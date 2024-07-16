# About

Socktool is like `ss(8)` with some extra features:

* Look for netns inside following directories:
  * /var/run/netns
  * /var/run/docker/netns

# Disclaimer

I'm learning Rust while writing this tool, please apologize for my code.

# Install

* Use [rustup](https://rustup.rs/) to install rust
* Clone repository
* Run `cargo build` or `cargo build --release`


## Debian package

* Install [cargo-deb](https://crates.io/crates/cargo-deb)
* Run `cargo deb`

# Example

```
❯ sudo ./target/debug/socktool -l
 Family | Protocol | Source            | Destination | User | NetNS                              | PID   | Command
--------+----------+-------------------+-------------+------+------------------------------------+-------+----------------------------------------------------------------
 IPv4   | tcp      | [0.0.0.0]:3493    | [0.0.0.0]:0 | 0    | None                               | 1688  | /usr/lib/nut/upsd
 IPv4   | tcp      | [0.0.0.0]:2049    | [0.0.0.0]:0 | 0    | None                               | 0     | unknown
 IPv4   | tcp      | [0.0.0.0]:51753   | [0.0.0.0]:0 | 0    | None                               | 1709  | /usr/sbin/rpc.mountd
 IPv4   | tcp      | [0.0.0.0]:50613   | [0.0.0.0]:0 | 123  | None                               | 1705  | /usr/sbin/rpc.statd
 IPv4   | tcp      | [0.0.0.0]:111     | [0.0.0.0]:0 | 0    | None                               | 1     | /usr/lib/systemd/systemd
 IPv4   | tcp      | [0.0.0.0]:53      | [0.0.0.0]:0 | 0    | None                               | 1646  | /usr/sbin/dnsmasq
 IPv4   | tcp      | [0.0.0.0]:22      | [0.0.0.0]:0 | 0    | None                               | 1644  | /usr/sbin/sshd
 IPv4   | tcp      | [0.0.0.0]:8000    | [0.0.0.0]:0 | 0    | None                               | 0     | unknown
 IPv4   | tcp      | [0.0.0.0]:53615   | [0.0.0.0]:0 | 0    | None                               | 1709  | /usr/sbin/rpc.mountd
 IPv4   | tcp      | [0.0.0.0]:37677   | [0.0.0.0]:0 | 0    | None                               | 0     | unknown
 IPv4   | tcp      | [0.0.0.0]:43227   | [0.0.0.0]:0 | 0    | None                               | 1709  | /usr/sbin/rpc.mountd
 IPv4   | tcp      | [127.0.0.1]:2947  | [0.0.0.0]:0 | 0    | None                               | 1     | /usr/lib/systemd/systemd
 IPv4   | tcp      | [127.0.0.1]:631   | [0.0.0.0]:0 | 0    | None                               | 1620  | /usr/sbin/cupsd
 IPv4   | tcp      | [127.0.0.1]:25    | [0.0.0.0]:0 | 0    | None                               | 2055  | /usr/sbin/exim4
 IPv6   | tcp      | [::]:3493         | [::]:0      | 0    | None                               | 1688  | /usr/lib/nut/upsd
 IPv6   | tcp      | [::]:2049         | [::]:0      | 0    | None                               | 0     | unknown
 IPv6   | tcp      | [::]:34105        | [::]:0      | 0    | None                               | 0     | unknown
 IPv6   | tcp      | [::]:1716         | [::]:0      | 1000 | None                               | 2934  | /usr/lib/x86_64-linux-gnu/libexec/kdeconnectd
 IPv6   | tcp      | [::]:111          | [::]:0      | 0    | None                               | 1     | /usr/lib/systemd/systemd
 IPv6   | tcp      | [::]:53           | [::]:0      | 0    | None                               | 1646  | /usr/sbin/dnsmasq
 IPv6   | tcp      | [::]:22           | [::]:0      | 0    | None                               | 1644  | /usr/sbin/sshd
 IPv6   | tcp      | [::]:40445        | [::]:0      | 0    | None                               | 1709  | /usr/sbin/rpc.mountd
 IPv6   | tcp      | [::]:8000         | [::]:0      | 0    | None                               | 0     | unknown
 IPv6   | tcp      | [::]:55405        | [::]:0      | 0    | None                               | 1709  | /usr/sbin/rpc.mountd
 IPv6   | tcp      | [::]:37975        | [::]:0      | 0    | None                               | 1709  | /usr/sbin/rpc.mountd
 IPv6   | tcp      | [::]:59433        | [::]:0      | 123  | None                               | 1705  | /usr/sbin/rpc.statd
 IPv6   | tcp      | [::1]:2947        | [::]:0      | 0    | None                               | 1     | /usr/lib/systemd/systemd
 IPv6   | tcp      | [::1]:25          | [::]:0      | 0    | None                               | 2055  | /usr/sbin/exim4
 IPv6   | tcp      | [::1]:631         | [::]:0      | 0    | None                               | 1620  | /usr/sbin/cupsd
 IPv4   | tcp      | [0.0.0.0]:8000    | [0.0.0.0]:0 | 0    | /var/run/docker/netns/8574331c6b8f | 0     | unknown
```
