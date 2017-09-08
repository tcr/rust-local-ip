extern crate regex;

use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use regex::Regex;

#[cfg(not(windows))]
pub fn get() -> Option<IpAddr> {
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        if let Some(host) = cap.get(2) {
            if host.as_str() != "127.0.0.1" {
                if let Ok(addr) = host.as_str().parse::<Ipv4Addr>() {
                    return Some(IpAddr::V4(addr))
                }
                if let Ok(addr) = host.as_str().parse::<Ipv6Addr>() {
                    return Some(IpAddr::V6(addr))
                }
            }
        }
    }

    None
}

#[cfg(windows)]
pub fn get() -> Option<IpAddr> {
    let output = Command::new("ipconfig").output().expect("Command IPCONFIG failed.");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"IPv4 Address[.\s]*:\s*([\d.]+)\s*"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        if let Some(host) = cap.get(1) {
            if host.as_str() != "127.0.0.1" {
                if let Ok(addr) = host.as_str().parse::<Ipv4Addr>() {
                    return Some(IpAddr::V4(addr))
                }
            }
        }
    }

    let re = Regex::new(r#"IPv6 Address[.\s]*:\s*([a-fA-F\d:]+)\s*"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        if let Some(host) = cap.get(1) {
            if host.as_str() != "::1" {
                if let Ok(addr) = host.as_str().parse::<Ipv6Addr>() {
                    return Some(IpAddr::V6(addr))
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
