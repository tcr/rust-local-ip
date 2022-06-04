extern crate regex;

use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;

pub fn get() -> Option<IpAddr> {
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        if let Some(host) = cap.at(2) {
            if host != "127.0.0.1" {
                if let Ok(addr) = host.parse::<Ipv4Addr>() {
                    return Some(IpAddr::V4(addr));
                }
                if let Ok(addr) = host.parse::<Ipv6Addr>() {
                    return Some(IpAddr::V6(addr));
                }
            }
        }
    }

    None
}

pub fn get_iface_addr(iface: &str) -> Option<IpAddr> {
    let output = Command::new("ifconfig")
        .arg(iface)
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    let re = Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#).unwrap();
    for cap in re.captures_iter(&stdout) {
        if let Some(host) = cap.at(2) {
            if host != "127.0.0.1" {
                if let Ok(addr) = host.parse::<Ipv4Addr>() {
                    return Some(IpAddr::V4(addr));
                }
                if let Ok(addr) = host.parse::<Ipv6Addr>() {
                    return Some(IpAddr::V6(addr));
                }
            }
        }
    }

    None
}

pub fn get_ifaces() -> Vec<String> {
    let output = Command::new(format!("ls"))
        .arg("/sys/class/net")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = String::from_utf8(output.stdout).unwrap();

    //let re = Regex::new(r#"(?m)^?(([a-z][0-9]).*).*$"#).unwrap();
    let re = Regex::new(r#".*"#).unwrap();
    let mut ifaces = vec!["".to_string(); 0];
    for cap in re.captures_iter(&stdout) {
        if let Some(host) = cap.at(0) {
            if cap.at(0).unwrap() != "" {
                ifaces.push(host.to_string());
            }
        }
    }

    ifaces
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
