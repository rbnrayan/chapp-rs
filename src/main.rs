use std::env;
use std::process::exit;

mod cli_arg;
use cli_arg::TypeOption;

// mod thread_pool;

mod chapp_server;
mod chapp_client;
use chapp_server::ChappServer;
use chapp_client::ChappClient;

fn main() {
    let args: Vec<String> = env::args().collect();

    match cli_arg::parse_args(args).unwrap() {
        TypeOption::LISTEN(ip) => {
            ChappServer::new(&ip.to_string()).listen().unwrap();
        },
        TypeOption::READONLY(ip) => {
            let mut reader = ChappClient::connect(ip);
            reader.send(b"READER").unwrap();
        }
        TypeOption::WRITEONLY(ip) => {
            let mut writer = ChappClient::connect(ip);
            writer.send(b"WRITER").unwrap();

            let mut buf = String::new();

            while &buf[..] != "q" {
                std::io::stdin().read_line(&mut buf).unwrap();
                writer.send(buf.as_bytes()).unwrap();
                buf.clear();
            }
        },
    };
}

