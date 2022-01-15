use std::{net::IpAddr, str::FromStr};

use scrust::scan;
fn main() {
    let ip = IpAddr::from_str("192.168.1.11").unwrap();
    let init_port = 70;
    let end_port = 85;
    scan(ip, init_port, end_port);        
}
