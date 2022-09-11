mod utils;

use utils::service_parser::service;
use utils::port_state::{PortState};

use tokio::{net::TcpStream};
use tokio::time::{Instant, timeout_at};
use std::time::Duration;
use std::net::IpAddr;
use tabled::Tabled;

#[derive(Tabled)]
pub struct ScrustOutput {
    port: u16,
    state: String,
    service: String
}

pub async fn scan(ip: IpAddr, init_port: u16, end_port: u16) -> Vec<ScrustOutput> {
    let mut table_data: Vec<ScrustOutput> = vec![];
    let (sender, mut receiver) = tokio::sync::mpsc::channel(1024);
    for port in init_port..=end_port {
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            let result: PortState = port_connection(ip, port).await;
            let _ = sender_clone.send((port, result)).await;
        });
    }
    
    drop(sender);
    while let Some(value) = receiver.recv().await {
        match value {
            (_, PortState::Closed) => (),
            (port, state) => {
                table_data.push(ScrustOutput {
                    port,
                    state: state.colorize(state.to_string()),
                    service: service(port)
                });
            }
        };
    };

    table_data
}

async fn port_connection(ip: IpAddr, port: u16) -> PortState {
    let stream = TcpStream::connect((ip, port)).await;
    match stream {
        Ok(_) => PortState::Open,
        Err(_) => PortState::Closed
    }
}