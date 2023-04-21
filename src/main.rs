use std::{env, process::exit};
use std::io::{self, Write};

mod cli_arg;
use cli_arg::TypeOption;

mod chapp_server;
mod chapp_client;
use chapp_server::ChappServer;
use chapp_client::ChappClient;

fn main() {
    let args: Vec<String> = env::args().collect();

    match cli_arg::parse_args(args).unwrap() {
        TypeOption::LISTEN(ip) => {
            if let Err(e) = ChappServer::new(&ip.to_string()).listen() {
                eprintln!("SERVER: failed to listen on {:?}: {:?}", ip, e);
                exit(1);
            }
        },
        TypeOption::READONLY(ip) => {
            let mut reader = ChappClient::connect(ip);

            if let Err(e) = reader.send(b"READER") {
                eprintln!("READONLY client: failed to send message: {:?}", e);
                exit(1);
            }

            loop {
                match reader.read() {
                    Ok((bytes_read, data)) => {
                        if bytes_read == 0 {
                            println!("Stream closed");
                            break;
                        } else {
                            // RECEIVED DATA:
                            // <ip_address>:<port>\ndata\n[\0..]
                            let mut data_split = data.split(|c| *c == b'\n');

                            let from: &[u8] = data_split.next().unwrap();
                            let data: &[u8] = data_split.next().unwrap();

                            let from = String::from_utf8(from.to_vec()).unwrap();
                            let stringified_data = String::from_utf8(data.to_vec()).unwrap();
                            let stringified_data = stringified_data.strip_suffix("\n")
                                .unwrap_or(&stringified_data);

                            let msg = format!("@[{}]: {}", from, stringified_data);

                            println!(" > {}", msg);
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read bytes: {:?}", e);
                        exit(1);
                    },
                }
            }
        }
        TypeOption::WRITEONLY(ip) => {
            let mut writer = ChappClient::connect(ip);
            
            if let Err(e) = writer.send(b"WRITER") {
                eprintln!("WRITEONLY client: failed to send message: {:?}", e);
                exit(1);
            }

            let mut buf = String::new();

            while &buf[..] != "q" {
                print!(" > ");

                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut buf).expect("Failed to read line from stdin");

                if let Err(e) = writer.send(buf.as_bytes()) {
                    eprintln!("WRITEONLY client: failed to send message: {:?}", e);
                    exit(1);
                }

                buf.clear();
            }
        },
    };
}

