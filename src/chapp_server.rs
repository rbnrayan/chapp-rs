// use crate::chapp_client::ChappWriteClient;
use std::io::Read;
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::{Arc, /*Mutex*/};

pub struct ChappServer {
    addr: SocketAddr,
    // clients: Mutex<Vec<ChappWriteClient>>,
}

impl ChappServer {
    pub fn new(addr: &str) -> Self {
        ChappServer {
            addr: addr.parse().expect("Failed to parse socket address"),
            // clients: Mutex::new(Vec::with_capacity(10)),
        }
    }

    pub fn listen(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.addr)
                .expect(format!("Failed to listen on: {:?}", self.addr).as_ref());

        println!("Server listening on: @[{:?}]\n", self.addr);

        let mut self_local = Arc::new(self);

        for stream in listener.incoming() {
            let self_clone = Arc::clone(&mut self_local);
            let stream = stream?;

            std::thread::spawn(move || {
                self_clone.handle_conn(stream);
            });
        }

        Ok(())
    }

    fn handle_conn(&self, mut stream: TcpStream) {
        let stream_addr = stream.peer_addr().unwrap();
        println!("\n  ~ + @[{:?}] ~\n", stream_addr);

        let mut type_buf: [u8; 6] = [0; 6];
        if let Ok(6) = stream.read(&mut type_buf) {
            match &type_buf {
                b"WRITER" => {
                    // self.handle_writer();
                },
                b"READER" => {
                    // self.handle_reader();
                },
                _ => {
                    eprintln!("Unknown connection type");
                    return ();
                },
            }
        } else {
            eprintln!("Unexpected connection");
            return ();
        }

        loop {
            let mut buf: [u8; 1024] = [0; 1024];

            if let Ok(bytes_read) = stream.read(&mut buf) {
                if bytes_read == 0 {
                    println!("\n  ~ - @[{:?}] ~\n", stream_addr);
                    break;
                } else {
                    let stringified_data = String::from_utf8(buf[..bytes_read].to_vec()).unwrap();
                    println!("@[{:?}] > {}", stream_addr,
                                             stringified_data.strip_suffix("\n").unwrap_or(&stringified_data));
                }
            } else {
                println!("\n  ~ - @[{:?}] ~\n", stream_addr);
            }
        }
    }
}
