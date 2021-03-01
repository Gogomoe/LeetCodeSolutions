impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        if ip.contains(":") {
            if verify_ipv6(ip) {
                String::from("IPv6")
            } else {
                String::from("Neither")
            }
        } else {
            if verify_ipv4(ip) {
                String::from("IPv4")
            } else {
                String::from("Neither")
            }
        }
    }
}

fn verify_ipv4(ip: String) -> bool {
    let split: Vec<&str> = ip.split(".").collect();
    if split.len() != 4 { return false; }
    for it in split.iter() {
        let num = i32::from_str_radix(*it, 10).unwrap_or(-1);
        if !(num >= 0 && num <= 255 && num.to_string() == *it) {
            return false;
        }
    }
    true
}

fn verify_ipv6(ip: String) -> bool {
    let split: Vec<&str> = ip.split(":").collect();
    if split.len() != 8 { return false; }
    for it in split.iter() {
        if it.len() > 4 { return false; }
        let num = i32::from_str_radix(*it, 16).unwrap_or(-1);
        if !(num >= 0 && num <= 65535) {
            return false;
        }
    }
    true
}

struct Solution {}

fn main() {}