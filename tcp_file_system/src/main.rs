use std::str;
use std::io::prelude::*;
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
    let locat = String::from("~/remote/");

    loop {
        let mut byte_read = stream.read(&mut command_buff).expect("Failed in receiving command!");
        if let 0 = byte_read {
            stream.shutdown(std::net::Shutdown::Both).expect("Shutdown connection failed!");
            return Ok(());
        }

        match str::from_utf8(&command_buff[..byte_read]).unwrap() {
            "ls" => {
                let response = Command::new("ls")
                                            .arg("-l")
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
            }
            _ => {
                stream.write("You have entered a wrong command!".as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
            }
        }
    }
}

fn client(addr: &SocketAddr) -> Result<(), std::io::Error> {
    Ok(())
}