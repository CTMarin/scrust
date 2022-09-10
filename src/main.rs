use std::{net::IpAddr, str::FromStr};
use clap::{Parser, IntoApp, ErrorKind};
use scrust::scan;

#[derive(Parser, Debug)]
#[clap(author = "CTMarin", version = "0.4.0", about = "A simple port scanner written in Rust", long_about = None)]
struct Scrust {
    #[clap()]
    address: String,

    /// First port to scan
    #[clap(short = 'i', long = "initial-port", default_value_t = 1)]
    init_port: u16, 

    /// Last port to scan
    #[clap(short = 'e', long = "end-port", default_value_t = 65535)]
    end_port: u16,

    /// Print all filtered ports
    #[clap(short = 'f', long = "filtered")]
    filtered_ports: bool
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Scrust::parse();
    if args.init_port > args.end_port {
        Scrust::into_app().error(
            ErrorKind::InvalidSubcommand, 
            "End port must be greater than initial port").exit();
    }
    let ip = IpAddr::from_str(&args.address).unwrap();
    let init_port = args.init_port;
    let end_port = args.end_port;
    let filtered = args.filtered_ports;
    scan(ip, init_port, end_port, filtered).await;
}
