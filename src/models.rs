use std::net::IpAddr;

#[derive(Debug)]
pub struct ProcStat {
    pub pid: u16,
    pub cmdline: String,
    pub fds: Vec<u32>,
}

impl ProcStat {
    pub fn new(pid: u16, cmdline: String, fds: Vec<u32>) -> Self {
        ProcStat { pid, cmdline, fds }
    }
}

pub struct SocketStat {
    pub family: u8,
    // pub id: u16,
    pub protocol: u8,
    pub source_ip: IpAddr,
    pub source_port: u16,
    pub destination_ip: IpAddr,
    pub destination_port: u16,
    pub owner: u32,
    pub inode: u32,
    pub netns: Option<String>,
}

impl SocketStat {
    pub fn new(
        family: u8,
        // id: u16,
        protocol: u8,
        source_ip: IpAddr,
        source_port: u16,
        destination_ip: IpAddr,
        destination_port: u16,
        owner: u32,
        inode: u32,
    ) -> Self {
        SocketStat {
            family,
            // id,
            protocol,
            source_ip,
            source_port,
            destination_ip,
            destination_port,
            owner,
            inode,
            netns: None,
        }
    }
    pub fn setns(&mut self, netns: String) {
        self.netns = Some(netns);
    }
}
