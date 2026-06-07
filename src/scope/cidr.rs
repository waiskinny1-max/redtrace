use ipnet::IpNet;
use std::net::IpAddr;

pub fn ip_in_cidr(ip: &str, cidr: &str) -> bool {
    let Ok(ip_addr) = ip.parse::<IpAddr>() else {
        return false;
    };
    let Ok(network) = cidr.parse::<IpNet>() else {
        return false;
    };
    network.contains(&ip_addr)
}
