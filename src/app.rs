use std::net::TcpListener;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    port: u16,
}

pub fn get_port() -> u16 {
    Args::parse().port
}

pub fn is_port_in_use(port: u16) -> bool {
    match TcpListener::bind(("0.0.0.0", port)) {
        Ok(_) => false,
        Err(_) => true
    }
}
