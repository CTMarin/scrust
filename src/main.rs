use std::{net::IpAddr, str::FromStr};
use clap::{Parser, IntoApp, ErrorKind};
use tabled::{Table, Style};
use scrust::{scan, ScrustOutput};

#[derive(Parser, Debug)]
#[clap(author = "CTMarin", version = "0.4.0", about = "A simple port scanner written in Rust", long_about = None)]
struct Scrust {
    #[clap()]
    address: String,

    #[clap(short = 'i', long = "initial-port", default_value_t = 1)]
    init_port: u16, 

    #[clap(short = 'e', long = "end-port", default_value_t = 65535)]
    end_port: u16
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
    let table_data: Vec<ScrustOutput> = scan(ip, init_port, end_port).await;
    let table = Table::new(table_data).with(Style::rounded());
    println!("{}", table.to_string());
}
