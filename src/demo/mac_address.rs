use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;

fn get_mac_address() -> Option<String> {
    let output = Command::new("ipconfig")
        .arg("/all")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    for line in lines {
        if line.contains("Physical Address") {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() > 1 {
                return Some(parts[1].trim().to_string());
            }
        }
    }

    None
}

fn main() {
    match get_mac_address() {
        Some(mac) => println!("MAC Address: {}", mac),
        None => println!("Failed to retrieve MAC address"),
    }
}