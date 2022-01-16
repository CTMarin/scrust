use std::net::{IpAddr, TcpStream};

enum PortState {
    Open,
    Closed
} 

pub fn scan(ip: IpAddr, init_port: u16, end_port: u16) {
    for port in init_port..=end_port {
        let result: PortState = port_connection(ip, port);
        match result {
            PortState::Open => println!("Port {} is open", port),
            PortState::Closed => println!("Port {} is closed", port)
        }
    }
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