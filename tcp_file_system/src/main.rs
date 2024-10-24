use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use clap::Parser;

mod argument_parser;

fn main() -> std::io::Result<()> {
    let cli = argument_parser::Cli::parse();

    let addr = 
        if cli.actions.server {SocketAddr::from((Ipv4Addr::UNSPECIFIED, cli.port))}
        else {SocketAddr::from((IpAddr::V4(cli.address.parse().expect("Entered a wrong IPv4 address.")), cli.port))};
    
    Ok(())
}

fn server(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr);
    
    Ok(())
}