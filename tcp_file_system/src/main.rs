use core::str;
use std::io::{prelude::*, stdin};
use std::fs;
use std::process::Command;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use clap::Parser;

mod argument_parser;

fn main() -> std::io::Result<()> {
    let cli = argument_parser::Cli::parse();

    let addr = 
        if cli.actions.server {SocketAddr::from((Ipv4Addr::UNSPECIFIED, cli.port))}
        else {
            SocketAddr::from(
                (IpAddr::V4(cli.address.parse().expect("Entered a wrong IPv4 address.")), cli.port)
            )
        };
    
    if cli.actions.server {
        let listener = TcpListener::bind(&addr).expect("Failed to bind to address.");
        let file_lock = Arc::new(Mutex::new(0));

        loop {
            let (stream, source_addr) = 
                listener.accept().expect("Accepting remote connection failed!");
            println!("Estabilished connection with {source_addr}.");
            let file_lock = Arc::clone(&file_lock);
            thread::spawn(move || {
                server(stream, file_lock).expect("Failed to start server.");
            });
        };

    } else {
        client(&addr).expect("Failed to start a client");
    }
    Ok(())
}

/*
    file read and write are protected by a mutex lock in server.
 */
fn server(mut stream: TcpStream, file_lock: Arc<Mutex<i32>>) -> Result<(), std::io::Error> {
    let mut command_buff = [0; 16];
    let mut file_name_buff = [0; 32];
    let mut file_buff = [0; 2048];
    
    // Change this variable to change the folder visited by server.
    let locat = String::from("/tmp/remote/");

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

                let _lock = file_lock.lock().unwrap();
                fs::write(format!("{}{}", locat, str::from_utf8(&file_name_buff[..byte_read]).unwrap()), 
                    &file_buff[..content_length]).expect("Writing file error!");
            },

            "download" => {
                byte_read = stream.read(&mut file_name_buff).expect("Failed in receiving file name!");
                
                let file_read = {
                    let _lock = file_lock.lock().unwrap();
                    fs::read(format!("{}{}", locat, str::from_utf8(&file_name_buff[..byte_read]).unwrap()))
                };

                let file_read = match file_read {
                    Ok(fi) => fi,
                    Err(_) => String::from("!!Err!!").into_bytes(),
                };

                stream.write(&file_read).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
            },

            _ => {
                // As designated, client should not send unrecognizable command.
                // If it does occur, something wrong must happen during transferring command.
                // Error recover measurement haven't been implemented, shut this connection avoid further error.
                stream.shutdown(std::net::Shutdown::Both).expect("Shutdown connection failed!");
                println!("Connection closed due to previous error.");
                return Ok(());
            },
        }
    }
}

/*
    Client side is a simple command line interface.
    It supports "ls", "upload", "download", "shutdown" commands.
    file read/write in local folder is not lock in the client side, use it at your own risk.
 */
fn client(addr: &SocketAddr) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(addr).expect("Failed to connect to server!");
    println!("Established connection with server!");

    let mut input_buff = String::new();
    let mut buff = [0; 2048];

    // Change this variable to change the folder visited by client.
    // You cannot list the folder inside the client.
    let locat = String::from("/tmp/local/");

    loop {
        println!("Please enter a command:");
        input_buff.clear();
        stdin().read_line(&mut input_buff).expect("Reading line error!");

        match input_buff.trim() {
            "ls" => {
                stream.write("ls".as_bytes()).expect("Failed to write to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");

                let byte_read = stream.read(&mut buff).expect("Failed in receiving \"ls\" result!");
                if let 0 = byte_read {
                    println!("Connection closed by server.");
                    return Ok(());
                }

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

                let file_read = fs::read(format!("{}{}", locat, &input_buff));
                let file_read = match file_read {
                    Ok(fi) => fi,
                    Err(_) => {
                        println!("File not found or you have no right to read it.");
                        continue;
                    },
                };

                stream.write(&file_read).expect("Failed on writing to send buffer!");
                stream.flush().expect("Failed to flush send buffer!");
                println!("File have been sent to server.");
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
                if let 0 = byte_read {
                    println!("Connection closed by server.");
                    return Ok(());
                }
                
                match str::from_utf8(&buff[..byte_read]).unwrap() {
                    "!!Err!!" => {
                        println!("You have specified a wrong file name or you have no right to read it.");
                    },
                    _ => {
                        fs::write(format!("{}{}", locat, &input_buff), &buff[..byte_read])
                            .expect("Failed on writing file!");
                        println!("File successfully downloaded.");
                    },
                };
            },

            "shutdown" => {
                stream.shutdown(std::net::Shutdown::Write).expect("Failed to shut connection!");
                return Ok(());
            },

            _ => {
                println!("You have entered a unsupported command!");
            },
        };
    }
}
