use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::time::Duration;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use anyhow::Result;

struct Server {
    poll: Poll,
    listener: TcpListener,
    connections: HashMap<Token, TcpStream>,
    buffers: HashMap<Token, Vec<u8>>, // 新增：为每个连接维护缓冲区
    next_token: usize,
}

impl Server {
    fn new(address: SocketAddr) -> Result<Self> {
        let poll = Poll::new()?;
        let mut listener = TcpListener::bind(address)?;
        poll.registry()
            .register(&mut listener, Token(0), Interest::READABLE | Interest::
WRITABLE)?;

        Ok(Server {
            poll,
            listener,
            connections: HashMap::new(),
            buffers: HashMap::new(), // 初始化缓冲区
            next_token: 1,
        })
    }   

    fn run(&mut self) -> Result<()> {
        const SERVER: Token = Token(0);
        let mut events = Events::with_capacity(128);

        loop {
            self.poll.poll(&mut events, Some(Duration::from_millis(100)))?;

            for event in events.iter() {
                match event.token() {
                    SERVER => loop {
                        match self.listener.accept() {
                            Ok((connection, _)) => self.process_connection(connection)?,
                            Err(ref err) if would_block(err) => {
                                println!("No more connections to accept");
                                break
                            },
                            Err(err) => return Err(err.into()),
                        }
                    }
                    token => self.process_echo(token)?,
                }
            }
        }
    }

    fn process_connection(&mut self, mut connection: TcpStream) -> Result<()> {
        let token = Token(self.next_token);
        self.next_token += 1;
        self.poll.registry().register(&mut connection, token, Interest::READABLE | Interest::WRITABLE)?;
        self.connections.insert(token, connection);
        println!("New connection: {:?}", token);
        Ok(())
    }

    fn process_echo(&mut self, token: Token) -> Result<()> {
        let buf = self.buffers.entry(token).or_insert_with(Vec::new);

        if let Some(connection) = self.connections.get_mut(&token) {
            let mut temp = [0u8; 1024];
            match connection.read(&mut temp) {
                Ok(n) => {
                    buf.extend_from_slice(&temp[..n]);
                    if let Err(e) = connection.write_all(&temp[..n]) {
                        eprintln!("Failed to write to connection {:?}: {}", token, e);
                    }
                    println!("{}", String::from_utf8_lossy(&temp[..n]));
                    self.poll.registry().deregister(connection)?;
                    self.connections.remove(&token);
                    self.buffers.remove(&token);
                }
                Err(ref err) if would_block(err) => {
                    println!("Would block on connection {:?}", token);
                }
                Err(err) => {
                    eprintln!("Failed to read from connection {:?}: {}", token, err);
                    return Err(err.into());
                }
            }
        }
        Ok(())
    }

} 
fn main() -> Result<()> {
    Server::new(SocketAddr::from(([127, 0, 0, 1], 8080)))?.run()
}


fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

