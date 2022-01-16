use std::net::{IpAddr, TcpStream};
use colored::Colorize;

enum PortState {
    Open,
    Closed
} 

pub fn scan(ip: IpAddr, init_port: u16, end_port: u16) {
    let mut open_ports: Vec<u16> = Vec::new();
    for port in init_port..=end_port {
        let result: PortState = port_connection(ip, port);
        match result {
            PortState::Open => {
                open_ports.push(port);
                println!("{}", format!("Port {} is open", port).green())
            }
            PortState::Closed => println!("{}", format!("Port {} is closed", port).red())
        }
    }
    println!("------------- Open Ports -------------");
    println!("{:?}", open_ports);
}

fn port_connection(ip: IpAddr, port: u16) -> PortState {
    let stream = TcpStream::connect((ip, port));
    match stream {
        Ok(_) => PortState::Open,
        Err(_) => {
            PortState::Closed
        }
    }
}