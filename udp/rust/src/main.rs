use std::net::{Ipv4Addr, UdpSocket, SocketAddr};
use std::str;
use clap::{Parser, Args};
use std::{thread, time};

fn main() {
    let cli = Cli::parse();
    
    // Assign destination ip and port address.
    let addr = 
        if cli.actions.receive {SocketAddr::from((Ipv4Addr::UNSPECIFIED, cli.port))} 
        else {SocketAddr::new(cli.address.parse().unwrap(), cli.port)};

    // The tcp method is not been implemented yet.
    if cli.methods.udp {
        if cli.actions.receive {
            receiver(&addr);
        } else if cli.actions.send {
            sender(&addr, &cli.message, cli.number);
        }
    }
}


// Receiver(Server) function. This function won't end by itself and controlled by user.
fn receiver(addr: &SocketAddr) {
    let mut count: u32 = 0;
    let mut buff = [0; 1024];

    // In above, we set receiver's binding address as `UNSPECIFIED` and means it will listen from all ip address of 
    // port {cli.port}(default is 2000)
    let socket = UdpSocket::bind(&addr).expect("Error on binding a socket.");

    loop {
        let (number_of_byte, source_addr) = 
            socket.recv_from(&mut buff).expect("Error on receiving data.");
        count += 1;

        socket.send_to(&count.to_string().as_bytes(), source_addr).expect("Error on sending feedback");
        println!("{} From {source_addr}\nThis is the {count}th packet\n", str::from_utf8(&buff[..number_of_byte]).unwrap());
    }
}


// Sender(Client) function. This function will stop after sending {number} messages. If number didn't be specified 
// entering the program, it will use the default value: '0xffffff'.
fn sender(addr: &SocketAddr, msg: &String, number: usize) {
    let dur = time::Duration::new(1, 0);
    let mut buff = [0; 1024];

    // Let program itself to decide its address and port.
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Error on binding a socket.");

    for _i in 0..number {
        socket.send_to(msg.as_bytes(), addr).expect("Error on sending message.");
        println!("A message successfully sent!");

        let number_of_byte = socket.recv(&mut buff).expect("Error on receiving feedback.");
        println!("This is the {} message the server received.", str::from_utf8(&buff[..number_of_byte]).unwrap());
        
        thread::sleep(dur);
    }
}


// Command line argument parser. Use argument '-h' or '--help' to start the program to get more information.
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
    #[arg(short, long, requires = "Actions", default_value_t = String::from("127.0.0.1"))]
    address: String,

    /// Specify how many messages sent(Only used in sender mode)
    #[arg(short, default_value_t = 0xffffff)]
    number: usize,

    /// Specify the message you wish to send
    #[arg(short, long, default_value_t = String::from("Hello UDP!"))]
    message: String,

}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Methods {
    /// Use udp protocol
    #[arg(short, long)]
    udp: bool,

    /// Use tcp protocol(Not implemented yet)
    #[arg(short, long)]
    tcp: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Actions {
    /// To send
    #[arg(short, long)]
    send: bool,

    /// To receive
    #[arg(short, long)]
    receive: bool,
}