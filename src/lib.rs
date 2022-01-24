use tokio::{net::TcpStream};
use tokio::time::{Instant, timeout_at};
use std::time::Duration;
use std::net::IpAddr;
use colored::{Colorize, ColoredString};
use regex::{RegexBuilder};
use std::fmt;

enum PortState {
    Open,
    Closed,
    Filter
} 

impl PortState {
    fn colorize(&self, output: String) -> ColoredString {
        match *self {
            PortState::Open => output.green(),
            PortState::Closed => output.red(),
            PortState::Filter => output.yellow(),
        }
    }
}

impl fmt::Display for PortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PortState::Open => write!(f, "open"),
            PortState::Closed => write!(f, "closed"),
            PortState::Filter => write!(f, "filter")
        }
    }
}

pub async fn scan(ip: IpAddr, init_port: u16, end_port: u16) {
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
            (_, PortState::Closed) => (),
            (port, state) => {
                println!("{}", print_port(port, state).await);
            }
        };
    };
}

async fn print_port(port: u16, state: PortState) -> ColoredString {
    let port_info = format!("{}\t\t{}\t\t{}", port, state.to_string(), service(port).await);
    state.colorize(port_info)
}

async fn port_connection(ip: IpAddr, port: u16) -> PortState {
    // Mix for doing just one tcp stream:
    // Do the timeout function
    //  match timeout_result {
    //      Err(_) => filtered,
    //      Ok(Ok(_)) => open,
    //      Ok(Err()) => closed
    // 
    if let Err(_) = timeout_at(Instant::now() + Duration::from_millis(500), TcpStream::connect((ip, port))).await {
        return PortState::Filter;
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
    let caps = re.captures(&services);
    match caps {
        Some(results) => String::from(&results[1]),
        None =>  String::from("unwknown")
    }
}

#[cfg(target_family = "windows")]
async fn service(port: u16) -> String {
    // C:\Windows\System32\drivers\etc on Windows
    let content = tokio::fs::read("C:/Windows/System32/drivers/etc/services").await.unwrap();
    let services = String::from_utf8(content).unwrap();
    let pattern = RegexBuilder::new(format!("([a-zA-Z-]+)(\\s*{}/tcp)", port).as_str());
    let re = pattern.build().unwrap();
    let caps = re.captures(&services).unwrap();
    String::from(&caps[1])
}