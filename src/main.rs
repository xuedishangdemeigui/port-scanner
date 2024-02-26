use port_scanner;
use std::env;

fn main() {
    let config = port_scanner::parse_args(env::args());
    if config.help {
        print!("Usage:\nport-scanner -h host -p port1[-port2]\n\nExample:\n1) Scan 22 port of 127.0.0.1\nport-scanner -h 127.0.0.1 -p 22\n2) Scan the ports 8000-9000 of 127.0.0.1\nport-scanner -h 127.0.0.1 -p 8000-9000\n");
        return;
    }

    let port_is_open = port_scanner::run(config);
    if port_is_open.len() > 0 {
        println!("Ports: {:?} are listening.", port_is_open);
    } else {
        println!("None port is open.");
    }
}
