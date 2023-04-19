use std::net::SocketAddr;

pub enum TypeOption {
    LISTEN(SocketAddr),
    READONLY(SocketAddr),
    WRITEONLY(SocketAddr),
}

pub fn print_usage() {
    eprintln!(r#"
usage: cargo run -- <OPTION> <IP>
OPTIONS:
    -l --listen  start the server
    -r --read    start a readonly client
    -w --write   start a writeonly client"#);
}

pub fn parse_args(args: Vec<String>) -> Result<TypeOption, String> {
    if args.len() < 3 {
        print_usage();
        std::process::exit(1);
    }

    let args = &args[1..];

    let prefix = &args[0][..2];
    assert!(prefix.len() >= 2);

    let ip: SocketAddr = args[1].parse().unwrap();

    if prefix == "--" {
        let arg = &args[0][2..];
        match arg {
            "listen" => Ok(TypeOption::LISTEN(ip)),
            "read" => Ok(TypeOption::READONLY(ip)),
            "write" => Ok(TypeOption::WRITEONLY(ip)),
            _ => Err(format!("Unknown argument option: --{}", arg)),
        }
    } else if &prefix[..1] == "-" {
        let arg = &args[0][1..2];
        match arg {
            "l" => Ok(TypeOption::LISTEN(ip)),
            "r" => Ok(TypeOption::READONLY(ip)),
            "w" => Ok(TypeOption::WRITEONLY(ip)),
            _ => Err(format!("Unknown argument option: -{}", arg)),
        }
    } else {
        Err("Failed to parse argument option".to_string())
    }
}
