use clap::Parser;
use std::process::Command;
use netstat::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, SocketInfo, ProtocolSocketInfo};

// Arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    port: u16,
}

pub fn get_port() -> u16 {
    Args::parse().port
}

fn read_local_port(socket_info: &SocketInfo) -> Option<u16> {
    match &socket_info.protocol_socket_info {
        ProtocolSocketInfo::Udp(_) => None,
        ProtocolSocketInfo::Tcp(tcp_info) => Some(tcp_info.local_port)
    }
}

pub fn get_pid(port: u16) -> Option<Vec<u32>> {
    let sockets = get_sockets_info(AddressFamilyFlags::IPV4, ProtocolFlags::TCP).unwrap();
    
    sockets
        .iter()
        .find_map(|socket| read_local_port(socket).filter(|&p| p == port).map(|_| socket.associated_pids.clone()))
}

pub fn kill_port(pid: u32) -> Result<(), String> {
    let output = Command::new("kill").arg(format!("{}", pid)).output().unwrap();
    if output.status.success() {
        Ok(())
    } else {
        Err("Somethin went wrong".to_string())
    }
}
