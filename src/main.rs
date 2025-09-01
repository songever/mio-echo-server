use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use mio::net::TcpListener;
use mio::{Events, Interest, Poll};
use anyhow::Result;

fn main() -> Result<()> {
    let mut poll = Poll::new()?;

    let mut events = Events::with_capacity(128);

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let mut listener = TcpListener::bind(address)?;

    const SERVER: mio::Token = mio::Token(0);
    poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    match listener.accept() {
                        Ok((connection, address)) => {
                            println!("Got a connection from: {}", address);
                        },
                        Err(ref err) if would_block(err) => break,
                        Err(err) => return Err(err.into()),
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}
