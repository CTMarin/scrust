use tokio::{net::TcpStream};
use tokio::time::{Instant, timeout_at};
use std::time::Duration;
use std::net::IpAddr;
use colored::Colorize;
use regex::{RegexBuilder};
use std::fmt;

enum PortState {
    Open,
    Closed,
    Filtered
} 

impl fmt::Display for PortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PortState::Open => write!(f, "open"),
            PortState::Closed => write!(f, "closed"),
            PortState::Filtered => write!(f, "filtered")
        }
    }
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
    let separator = "\t\t";
    println!("{}", format!("Port{}State{}Service{}Version", separator, separator, separator).blue());
    while let Some(value) = reciever.recv().await {
        match value {
            // TODO: Separar los println! en una función aparte, aquí solo aplicar el color
            (port, PortState::Open) => {
                println!("{}", format!("{}\t\t{}\t\t{}", port, PortState::Open.to_string(), service(port).await).green());
            }
            (port, PortState::Filtered) => {
                println!("{}", format!("{}\t\t{}\t\t{}", port, PortState::Filtered.to_string(), service(port).await).yellow());
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

#[cfg(target_family = "unix")]
async fn service(port: u16) -> String {
    let content = tokio::fs::read("/etc/services").await.unwrap();
    let services = String::from_utf8(content).unwrap();
    let pattern = RegexBuilder::new(format!("([a-zA-Z-]+)(\\s*{}/tcp)", port).as_str());
    let re = pattern.build().unwrap();
    let caps = re.captures(&services).unwrap();
    String::from(&caps[1])
}

#[cfg(target_family = "windows")]
async fn service(port: u16) -> String {
    // C:\Windows\System32\drivers\etc on Windows
    let content = tokio::fs::read("C:/Windows/System32/drivers/etc/services on Windows").await.unwrap();
    let services = String::from_utf8(content).unwrap();
    let pattern = RegexBuilder::new(format!("([a-zA-Z-]+)(\\s*{}/tcp)", port).as_str());
    let re = pattern.build().unwrap();
    let caps = re.captures(&services).unwrap();
    String::from(&caps[1])
}