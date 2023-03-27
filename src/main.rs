use port_scanner;
use std::env;

fn main() {
    let config = port_scanner::parse_args(env::args());
    let port_is_open = port_scanner::run(config);
    println!("port: {:?} is open", port_is_open)
}
