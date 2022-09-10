mod utils;

use utils::service_parser::service;
use utils::port_state::{PortState, port_info};

use tokio::{net::TcpStream};
use tokio::time::{Instant, timeout_at};
use std::time::Duration;
use std::net::IpAddr;
use colored::Colorize;

pub async fn scan(ip: IpAddr, init_port: u16, end_port: u16) {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);
    for port in init_port..=end_port {
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let result: PortState = port_connection(ip, port).await;
            let _ = sender_clone.send((port, result)).await;
        });
    }
    
    drop(sender);
    println!("{}", format!("Port\t\tState\t\tService\t\tVersion").blue());
    while let Some(value) = receiver.recv().await {
        match value {
            (_, PortState::Closed) => (),
            (port, state) => {
                println!("{}", port_info(port, state, service(port)));
            }
        };
    };
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