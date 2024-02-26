/// lib.rs implements the logic of port scanning.
use std::net::TcpStream;

const HOST_FLAG: &str = "-h";
const PORT_FLAG: &str = "-p";
const HELP_FLAG: &str = "--help";

#[derive(Debug)]
pub struct Config {
    pub host: String,   // Target host.
    pub port: Vec<u32>, // The ports of target host need to be scanned.
    pub help: bool,     // Print help information and exit directly if help is true.
}

impl Config {
    pub fn new() -> Config {
        Config {
            host: String::from(""),
            port: vec![],
            help: false,
        }
    }
}

pub fn parse_args(mut args: std::env::Args) -> Config {
    args.next();

    let mut config = Config::new();

    loop {
        match args.next() {
            Some(arg) => {
                if arg.starts_with(HOST_FLAG) {
                    config.host = args
                        .next()
                        .expect("No host after '-h' flag, usage: ./port-scanner --help");
                } else if arg.starts_with(PORT_FLAG) {
                    let port = args
                        .next()
                        .expect("No port after '-p' flag, usage: ./port-scanner --help");
                    config.port = port
                        .split('-')
                        .map(|x| x.parse().expect("Port is invalid"))
                        .collect();
                } else if arg.starts_with(HELP_FLAG) {
                    config.help = true;
                    break;
                }
            }
            None => break,
        }
    }

    config
}

pub fn run(config: Config) -> Vec<u32> {
    let mut port_iter = config.port.iter();
    let begin_port = *port_iter.next().expect("please provide the port info");
    let end_port = match port_iter.next() {
        Some(port) => *port,
        None => begin_port + 1,
    };

    let mut result: Vec<u32> = vec![];
    for port in begin_port..end_port {
        let addr = format!("{}:{}", config.host, port);
        if let Ok(_) = TcpStream::connect(&addr) {
            result.push(port)
        }
    }

    result
}
