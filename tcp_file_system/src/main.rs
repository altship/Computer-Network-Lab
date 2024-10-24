use std::str;
use std::io::{prelude::*, stdin};
use std::fs;
use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use clap::Parser;

mod argument_parser;

fn main() -> std::io::Result<()> {
    let cli = argument_parser::Cli::parse();

    let addr = 
        if cli.actions.server {SocketAddr::from((Ipv4Addr::UNSPECIFIED, cli.port))}
        else {SocketAddr::from((IpAddr::V4(cli.address.parse().expect("Entered a wrong IPv4 address.")), cli.port))};
    

    if cli.actions.server {
        server(&addr).expect("Failed to start server.");
    } else {
        client(&addr).expect("Failed to start a client");
    }
    Ok(())
}

fn server(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let listener = TcpListener::bind(addr).unwrap();
    let (mut stream, source_addr) = 
        listener.accept().expect("Accepting remote connection failed!");
    println!("Estabilished connection with {source_addr}.");

    let mut command_buff = [0; 16];
    let mut file_name_buff = [0; 32];
    let mut file_buff = [0; 2048];
    let locat = String::from("/home/altship/remote/");

    loop {
        let mut byte_read = stream.read(&mut command_buff).expect("Failed in receiving command!");
        if let 0 = byte_read {
            stream.shutdown(std::net::Shutdown::Both).expect("Shutdown connection failed!");
            println!("Connection closed.");
            return Ok(());
        }

        match str::from_utf8(&command_buff[..byte_read]).unwrap() {
            "ls" => {
                let response = Command::new("ls")
                                            .arg("-al")
                                            .arg(&locat)
                                            .output()
                                            .expect("Failed to \"ls\"");
                stream.write(&response.stdout).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
            },

            "upload" => {
                byte_read = stream.read(&mut file_name_buff).expect("Failed in receiving file name!");
                let content_length = stream.read(&mut file_buff).expect("Failed in receiving file!");

                fs::write(format!("{}{}", locat, str::from_utf8(&file_name_buff[..byte_read]).unwrap()), 
                    &file_buff[..content_length]).expect("Writing file error!");
            },

            "download" => {
                byte_read = stream.read(&mut file_name_buff).expect("Failed in receiving file name!");
                let file_read = fs::read(format!("{}{}", locat, str::from_utf8(&file_name_buff[..byte_read]).unwrap()));

                let file_read = match file_read {
                    Ok(fi) => fi,
                    Err(_) => String::from("You typed a wrong file name or you have no right to read it!").into_bytes(),
                };
                stream.write(&file_read).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
            },

            _ => {
                stream.write("You have entered a wrong command!".as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
            },
        }
    }
}

fn client(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(addr).expect("Failed to connect to server!");
    println!("Established connection with server!");

    let mut input_buff = String::new();
    let mut buff = [0; 2048];
    let locat = String::from("/home/altship/recv/");

    loop {
        println!("Please enter a command:");
        input_buff.clear();
        stdin().read_line(&mut input_buff).expect("Reading line error!");

        match input_buff.trim() {
            "ls" => {
                stream.write("ls".as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
                let byte_read = stream.read(&mut buff).expect("Failed in receiving \"ls\" result!");
                print!("{}", str::from_utf8(&buff[..byte_read]).unwrap());
            },

            "upload" => {
                stream.write("upload".as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");

                println!("Please enter the file name:");
                input_buff.clear();
                stdin().read_line(&mut input_buff).expect("Failed on reading line!");
                input_buff = input_buff.trim().to_string();
                stream.write(&input_buff.as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed on flushing send buffer!");

                let file_read = 
                    fs::read(format!("{}{}", locat, &input_buff))
                        .expect("Failed to read file!");

                stream.write(&file_read).expect("Failed on writing to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
            },

            "download" => {
                stream.write("download".as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");

                println!("Please enter the file name:");
                input_buff.clear();
                stdin().read_line(&mut input_buff).expect("Failed on reading line!");
                input_buff = input_buff.trim().to_string();
                stream.write(&input_buff.as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed on flushing send buffer!");

                let byte_read = stream.read(&mut buff).expect("Failed on receiving file!");
                fs::write(format!("{}{}", locat, &input_buff), &buff[..byte_read])
                   .expect("Failed on writing file!");
            },

            "shutdown" => {
                stream.shutdown(std::net::Shutdown::Write).expect("Failed to shut connection!");
                return Ok(());
            },

            _ => {
                println!("You have entered a unsupported command!");
            },
        }
    }
}