mod models;
use crate::models::{ProcStat, SocketStat};

use clap::Parser;
use netlink_packet_core::{
    NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_DUMP, NLM_F_REQUEST,
};
use netlink_packet_sock_diag::{
    constants::*,
    inet::{ExtensionFlags, InetRequest, SocketId, StateFlags},
    SockDiagMessage,
};
use netlink_sys::{protocols::NETLINK_SOCK_DIAG, Socket, SocketAddr};

// use whoami;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

use anyhow::Result;
use nix::sched::{setns, CloneFlags};
use regex::Regex;
use std::fs::File;
use std::os::fd::AsFd;
use std::{fs, net::IpAddr};

#[derive(Parser, Debug)]
struct Args {
    // Display only sockets in LISTEN state
    #[arg(short, action, help = "Only sockets in LISTENING state")]
    listen: bool,

    // Display all sockets
    #[arg(short, action, help = "All sockets")]
    all: bool,
    // #[arg(short, long)]
    // port: u16,
}

fn socket_stats(family: u8, protocol: u8, state_filter: StateFlags) -> Vec<SocketStat> {
    let mut stats = Vec::new();

    let mut socket = Socket::new(NETLINK_SOCK_DIAG).unwrap();
    let _port_number = socket.bind_auto().unwrap().port_number();
    socket.connect(&SocketAddr::new(0, 0)).unwrap();

    let mut nl_hdr = NetlinkHeader::default();
    nl_hdr.flags = NLM_F_REQUEST | NLM_F_DUMP;
    let mut packet = NetlinkMessage::new(
        nl_hdr,
        SockDiagMessage::InetRequest(InetRequest {
            family: family,
            protocol: protocol,
            extensions: ExtensionFlags::empty(),
            states: state_filter,
            socket_id: SocketId::new_v4(),
        })
        .into(),
    );

    packet.finalize();

    let mut buf = vec![0; packet.header.length as usize];

    // Before calling serialize, it is important to check that the buffer in
    // which we're emitting is big enough for the packet, other
    // `serialize()` panics.
    assert_eq!(buf.len(), packet.buffer_len());

    packet.serialize(&mut buf[..]);

    if let Err(e) = socket.send(&buf[..], 0) {
        println!("SEND ERROR {e}");
    }

    let mut receive_buffer = vec![0; 4096];
    let mut offset = 0;

    while let Ok(size) = socket.recv(&mut &mut receive_buffer[..], 0) {
        loop {
            let bytes = &receive_buffer[offset..];
            let rx_packet = <NetlinkMessage<SockDiagMessage>>::deserialize(bytes).unwrap();
            // println!("<<< {rx_packet:?}");

            match rx_packet.payload {
                NetlinkPayload::Noop => {}
                NetlinkPayload::InnerMessage(SockDiagMessage::InetResponse(response)) => {
                    // println!("{:?}", response);
                    let header = response.header;
                    stats.push(SocketStat::new(
                        family,
                        protocol,
                        header.socket_id.source_address,
                        header.socket_id.source_port,
                        header.socket_id.destination_address,
                        header.socket_id.destination_port,
                        header.uid,
                        header.inode,
                    ));
                }
                NetlinkPayload::Done(_) => {}
                _ => break,
            }

            offset += rx_packet.header.length as usize;
            if offset == size || rx_packet.header.length == 0 {
                // offset = 0;
                break;
            }
        }
        break;
    }
    return stats;
}

// Get all sockets
fn get_all_sockets(state_filter: StateFlags) -> Vec<SocketStat> {
    let mut stats = Vec::new();
    stats.append(&mut socket_stats(AF_INET, IPPROTO_TCP, state_filter));
    stats.append(&mut socket_stats(AF_INET, IPPROTO_UDP, state_filter));
    stats.append(&mut socket_stats(AF_INET6, IPPROTO_TCP, state_filter));
    stats.append(&mut socket_stats(AF_INET6, IPPROTO_UDP, state_filter));
    return stats;
}

fn get_proto(protocol: u8) -> String {
    match protocol {
        0 => String::from("ip"),
        6 => String::from("tcp"),
        17 => String::from("udp"),
        _ => String::from(format!("{}", protocol)),
    }
}

fn format_addr(family: u8, address: IpAddr, port: u16) -> String {
    match family {
        4 => String::from(format!("{}:{}", address, port)),
        _ => String::from(format!("[{}]:{}", address, port)),
    }
}

fn parse_processus_sockets(pid: u16) -> Vec<u32> {
    // Extract sockets ID from /proc/<pid>/fd directory
    // return a vector of sockets ID
    let socket_re = Regex::new(r"^socket:\[([0-9]+)\]").unwrap();

    let mut sockets = Vec::new();

    // read contents of /proc/<pid>/fd to extract sockets
    if let Ok(fds) = fs::read_dir(String::from(format!("/proc/{}/fd", pid))) {
        for fd in fds {
            let fd = fd.unwrap();
            // Entries in /proc/<pid>/fd/<fd> are symbolink links, we must read the target
            if let Ok(buffer) = fs::read_link(fd.path()) {
                // Check if target is like socket:[inode]
                let link_target = buffer.as_path().display().to_string();
                let captures = socket_re.captures(&link_target);
                match captures {
                    Some(x) => {
                        sockets.push(
                            String::from(x.get(1).unwrap().as_str())
                                .parse::<u32>()
                                .unwrap(),
                        );
                    }
                    None => {}
                }
            }
        }
    }

    sockets
}

fn parse_processus() -> Vec<ProcStat> {
    // Read content of /proc directory of extrat sockets from each directory
    let mut processus = Vec::new();

    let entries = fs::read_dir("/proc").unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        // Check if name of match a u16
        match name.parse::<u16>() {
            Ok(pid) => {
                // // Check if we read permission of directoy
                // let entry_name = "/proc/".to_owned() + &pid.to_string();
                // let entry_permissions = fs::metadata(entry_name).unwrap().permissions();
                // if entry_permissions.readonly()
                let sockets = parse_processus_sockets(pid);

                // read link of /proc/<pid>/exe if possible
                let exec_filename = String::from(format!("/proc/{}/exe", pid));
                let exec = fs::read_link(exec_filename);
                match exec {
                    Ok(_) => {
                        processus.push(ProcStat::new(
                            pid,
                            exec.unwrap().as_path().display().to_string(),
                            sockets,
                        ));
                    }
                    Err(_) => {
                        processus.push(ProcStat::new(pid, String::from("[unknown]"), sockets));
                    }
                }
            }
            Err(_) => {}
        }
    }
    // println!("{}", processus.len());
    // dbg!("{}", processus);

    processus
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut state_filter = StateFlags::ESTABLISHED;

    if args.listen {
        state_filter = StateFlags::LISTEN;
    };

    if args.all {
        state_filter = StateFlags::all();
    }

    // if whoami::username() != "root" {
    //     println!("Warning! Running as non root disable some features\n");
    // }

    let processus = parse_processus();
    // dbg!("{}", processus);
    // return Ok(());

    let mut output = Table::new();
    output.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    output.set_titles(row![
        "Family",
        "Protocol",
        "Source",
        "Destination",
        "User",
        "NetNS",
        "PID",
        "Command",
    ]);

    // Store all sockets
    let mut stats = Vec::new();

    /* Collect sockets from host */
    stats.append(&mut get_all_sockets(state_filter));

    // List netns from official directory, and docker directory
    let netns_paths: [&str; 2] = ["/var/run/netns", "/var/run/docker/netns"];

    /* Collect sockets from namespaces */
    for netns_path in netns_paths {
        let mut buffer = Vec::new();
        if !fs::metadata(netns_path).is_ok() {
            // println!("{} doesn't exists", netns_path);
            continue;
        }

        let paths = fs::read_dir(netns_path)?;

        for path in paths {
            let path = path.unwrap().path();
            let netns_fd = File::open(path.clone())?;
            let _ = setns(netns_fd.as_fd(), CloneFlags::CLONE_NEWNET);

            for mut s in get_all_sockets(state_filter) {
                s.setns(path.display().to_string());
                stats.push(s);
            }
        }
        stats.append(&mut buffer);
    }

    /* Finally print collected stats */
    for stat in stats {
        let process = processus.iter().find(|x| x.fds.contains(&stat.inode));
        let (process_name, process_pid) = match process {
            Some(x) => (x.cmdline.clone(), x.pid),
            None => (String::from("unknown"), 0 as u16),
        };

        let netns = match stat.netns {
            Some(x) => x,
            None => String::from("None"),
        };

        output.add_row(row![
            match stat.family {
                2 => "IPv4".to_owned(),
                10 => "IPv6".to_owned(),
                _ => String::from(format!("{}", stat.family)),
            },
            get_proto(stat.protocol),
            format_addr(stat.family, stat.source_ip, stat.source_port),
            format_addr(stat.family, stat.destination_ip, stat.destination_port),
            stat.owner,
            &format!("{}", netns),
            process_pid,
            &format!("{}", process_name)
        ]);
    }

    // stat.inode
    // processus.iter().map(|proc| proc.fds.iter())

    // stat.inode = 4242

    // processus = [
    //     { "pid": xxx, "comandline": "poetry", fds: [4242], }

    // ]

    // processus.iter().map(|process|
    //     if process.fds
    // );

    output.printstd();
    Ok(())
}
