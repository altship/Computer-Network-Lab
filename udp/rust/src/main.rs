use std::{env, io};
use std::net::{Ipv4Addr, UdpSocket, SocketAddr};
use std::str;
use clap::{Parser, Args};

#[derive(Parser)]
#[command(version, author = "Norton", about, long_about = None)]
struct Cli {
    #[command(flatten)]
    methods: Methods,

    #[command(flatten)]
    actions: Actions,

    /// Specify the port for communication
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 2000)]
    port: u16,

    /// Specify the IPv4 destination send for
    #[arg(short, long, requires = "send", default_value_t = String::from("127.0.0.1"))]
    address: String,

}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Methods {
    #[arg(short, long)]
    udp: bool,

    #[arg(short, long)]
    tcp: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Actions {
    #[arg(short, long)]
    send: bool,

    #[arg(short, long)]
    receive: bool,
}

fn main() {
    let cli = Cli::parse();
    
    let addr = if cli.actions.receive {SocketAddr::from((Ipv4Addr::UNSPECIFIED, cli.port))} else {};
}

fn receiver(addr: &SocketAddr) -> io::Result<()> {
    let socket = UdpSocket::bind(&addr).unwrap();
    loop {
        let mut buff = [0; 1024];
        let (number_of_byte, source_addr) = socket.recv_from(&mut buff).expect("Error on receiving data.");
        println!("{}From {source_addr}", str::from_utf8(&buff[..number_of_byte]).unwrap());
    }
}