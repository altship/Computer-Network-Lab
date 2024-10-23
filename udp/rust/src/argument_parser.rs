use clap::{Parser, Args};

// Command line argument parser. Use argument '-h' or '--help' to start the program to get more information.
#[derive(Parser)]
#[command(version, author = "Norton", about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub methods: Methods,

    #[command(flatten)]
    pub actions: Actions,

    /// Specify the port for communication
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 2000)]
    pub port: u16,

    /// Specify the IPv4 destination send for
    #[arg(short, long, requires = "Actions", default_value_t = String::from("127.0.0.1"))]
    pub address: String,

    /// Specify how many messages sent(Only used in sender mode)
    #[arg(short, value_parser = clap::value_parser!(u32).range(1..), default_value_t = 0xffffff)]
    pub number: u32,

    /// Specify the message you wish to send
    #[arg(short, long, default_value_t = String::from("Hello UDP!"))]
    pub message: String,

}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Methods {
    /// Use udp protocol
    #[arg(short, long)]
    pub udp: bool,

    /// Use tcp protocol(Not implemented yet)
    #[arg(short, long)]
    pub tcp: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Actions {
    /// To send
    #[arg(short, long)]
    pub send: bool,

    /// To receive
    #[arg(short, long)]
    pub receive: bool,
}