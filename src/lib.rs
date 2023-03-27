use std::net::TcpStream;

const HOST_FLAG: &str = "-h";
const PORT_FLAG: &str = "-p";

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: Vec<u32>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            host: String::from(""),
            port: vec![],
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
                    config.host = args.next().expect("there is no host info after '-h' flag");
                } else if arg.starts_with(PORT_FLAG) {
                    let port = args.next().expect("there is no port info after '-p' flag");
                    config.port = port
                        .split('-')
                        .map(|x| x.parse().expect("the port info is invalid"))
                        .collect();
                }
            }
            None => break,
        }
    }

    config
}

pub fn run(config: Config) -> Vec<u32> {
    let mut port_iter = config.port.iter();
    let begin_port = port_iter.next().expect("please provide the port info");
    let end_port = match port_iter.next() {
        Some(port) => port,
        None => begin_port,
    };

    let mut result: Vec<u32> = vec![];
    for port in *begin_port..*end_port {
        let addr = format!("{}:{}", config.host, port);
        if let Ok(_) = TcpStream::connect(&addr) {
            result.push(port)
        }
    }

    result
}
