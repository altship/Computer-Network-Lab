use clap::{Parser, Args};

// Command line argument parser. Use argument '-h' or '--help' to start the program to get more information.
#[derive(Parser)]
#[command(version, author = "Norton", about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub actions: Actions,

    /// Specify the port for communication
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..), default_value_t = 2000)]
    pub port: u16,

    /// Specify the IPv4 destination send for
    #[arg(short, long, requires = "Actions", default_value_t = String::from("127.0.0.1"))]
    pub address: String,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Actions {
    /// Run as server
    #[arg(short, long)]
    pub server: bool,

    /// Run as client
    #[arg(short, long)]
    pub client: bool,
}