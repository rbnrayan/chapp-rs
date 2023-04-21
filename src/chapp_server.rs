use crate::ChappClient;
use std::io::Read;
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::{Arc, Mutex};

pub struct ChappServer {
    addr: SocketAddr,
    clients: Mutex<Vec<ChappClient>>,
}

impl ChappServer {
    pub fn new(addr: &str) -> Self {
        ChappServer {
            addr: addr.parse().expect("Failed to parse socket address"),
            clients: Mutex::new(Vec::with_capacity(10)),
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

            std::thread::spawn(move || -> Result<(), String> {
                self_clone.handle_conn(stream)
            });
        }

        Ok(())
    }

    fn handle_conn(&self, mut stream: TcpStream) -> Result<(), String> {
        let mut type_buf: [u8; 6] = [0; 6];

        match stream.read(&mut type_buf) {
            Ok(_) => Ok(
                match &type_buf {
                    b"WRITER" => {
                        self.handle_writer(stream)?;
                    },
                    b"READER" => {
                        self.handle_reader(stream)?;
                    },
                    conn_type => {
                        return Err(format!("Unknown connection type: {:?}",
                            String::from_utf8(conn_type.to_vec()).unwrap()));
                    },
                }
            ),
            Err(e) => {
                return Err(format!("Unexpected connection: {:?}", e));
            },
        }
    }

    // MESSAGE:
    // 15 Bytes: ip address (255.255.255.255)
    //  5 Bytes: port       (:8080)
    //  2 Bytes: end        (\n\0)
    fn handle_writer(&self, mut stream: TcpStream) -> Result<(), String> {
        let stream_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                return Err(format!("Failed to get the stream addr: {:?}", e));
            }
        };

        println!("\n  ~ + @[{:?}]:WRITER ~\n", stream_addr);

        loop {
            let mut buf: [u8; 1024] = [0; 1024];

            match stream.read(&mut buf) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        println!("\n  ~ - @[{:?}] ~\n", stream_addr);
                        break;
                    } else {
                        let stringified_data = String::from_utf8(buf[..bytes_read].to_vec()).unwrap();
                        let stringified_data = stringified_data.strip_suffix("\n")
                            .unwrap_or(&stringified_data);

                        println!("@[{:?}]: {:?}", stream_addr, stringified_data);

                        let mut clients_lock = self.clients.lock().unwrap();

                        let addr = format!("{}\n", stream_addr.to_string());

                        if bytes_read + addr.len() > 1024 {
                            return Err(String::from("Can't concatenate addr + msg: BufferOverflow"));
                        }

                        let msg = format!("{}{}", addr, stringified_data);
                        let data = msg.as_bytes();

                        for client in clients_lock.iter_mut() {
                            client.send(data).unwrap();
                        }
                    }
                },
                Err(e) => {
                    return Err(format!("Failed to read from stream: {:?}", e));
                }
            }
        }

        Ok(())
    }

    fn handle_reader(&self, mut stream: TcpStream) -> Result<(), String> {
        let stream_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                return Err(format!("Failed to get the stream addr: {:?}", e));
            }
        };

        println!("\n  ~ + @[{:?}]:READER ~\n", stream_addr);

        let client_pos = self.clients.lock().unwrap().len();

        {
            let mut clients_lock = self.clients.lock().unwrap();
            let stream_clone = match stream.try_clone() {
                Ok(clone) => clone,
                Err(e) => {
                    return Err(format!("Failed to clone stream: {:?}", e));
                }
            };
            clients_lock.push(ChappClient::new(stream_clone));
        }

        let mut buf: [u8; 1024] = [0; 1024];

        loop {
            if let Ok(bytes_read) = stream.read(&mut buf) {
                if bytes_read == 0 {
                    let mut clients_lock = self.clients.lock().unwrap();
                    clients_lock.remove(client_pos);

                    println!("\n  ~ - @[{:?}] ~\n", stream_addr);
                    break;
                }
            } else {
                return Err(format!("Failed to read from @[{:?}]", stream_addr));
            }
        }

        Ok(())
    }
}
