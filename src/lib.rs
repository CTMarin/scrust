use tokio::{net::TcpStream};
use tokio::time::{Instant, timeout_at};
use std::time::Duration;
use std::net::IpAddr;
use colored::Colorize;

enum PortState {
    Open,
    Closed,
    Filtered
} 

pub async fn scan(ip: IpAddr, init_port: u16, end_port: u16) {
    //let mut open_ports: Vec<u16> = Vec::new();
    //let mut filtered_ports: Vec<u16> = Vec::new();
    let (sender, mut reciever) = tokio::sync::mpsc::channel(1024); 
    for port in init_port..=end_port {
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let result: PortState = port_connection(ip, port).await;
            let _ = sender_clone.send((port, result)).await;
        });
    }
    
    std::mem::drop(sender);
    println!("{}", format!("Port\t\tState\t\tService\t\tVersion").blue());
    while let Some(value) = reciever.recv().await {
        match value {
            (port, PortState::Open) => {
                println!("{}", format!("{}\t\topen", port).green());
            }
            (port, PortState::Filtered) => {
                println!("{}", format!("{}\t\tfiltered", port).yellow());
            }
            (_, _) => ()
        }
    }
}

async fn port_connection(ip: IpAddr, port: u16) -> PortState {
    if let Err(_) = timeout_at(Instant::now() + Duration::from_millis(500), TcpStream::connect((ip, port))).await {
        return PortState::Filtered;
    }

    let stream = TcpStream::connect((ip, port)).await;
    match stream {
        Ok(_) => PortState::Open,
        Err(_) => PortState::Closed
    }
}