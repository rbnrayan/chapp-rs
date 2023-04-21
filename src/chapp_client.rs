use std::net::{TcpStream, SocketAddr, ToSocketAddrs};
use std::io::{Read, Write};

#[derive(Debug)]
pub struct ChappClient {
    tcp_stream: TcpStream,
}

impl ChappClient {
    pub fn new(stream: TcpStream) -> Self {
        ChappClient {
            tcp_stream: stream,
        }
    }

    pub fn connect<T: ToSocketAddrs>(addr: T) -> Self {
        let socket_addr = 
            addr.to_socket_addrs().unwrap().next().unwrap();

        ChappClient::new(
            TcpStream::connect(socket_addr)
                .expect(&format!("Failed to connect to {:?}", socket_addr))
        )
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, String> {
        if let Ok(bytes_send) = self.tcp_stream.write(data) {
            if  data.len() != bytes_send {
                Ok(data.len() - bytes_send)
            } else {
                Ok(0)
            }
        } else {
            Err(format!("Failed to send bytes to {:?}", self.addr()))
        }
    }

    pub fn read(&mut self) -> Result<(usize, [u8; 1024]), String> {
        let mut buf: [u8; 1024] = [0; 1024];

        if let Ok(bytes_read) = self.tcp_stream.read(&mut buf) {
            if bytes_read > 1024 {
                Err(format!("Failed to read bytes from {:?}; Error: BufferOverflow", self.addr()))
            } else {
                Ok((bytes_read, buf))
            }
        } else {
            Err(format!("Failed to read bytes from {:?}; Error: `stream.read()` failed", self.addr()))
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.tcp_stream.peer_addr().unwrap()
    }
}
