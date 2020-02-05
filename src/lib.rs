use std::future::Future;
use std::io::{Error, ErrorKind, Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::TcpListener;
pub use mio::net::TcpStream;

pub mod http;

const SERVER_ACCEPT: Token = Token(0);
const SERVER: Token = Token(1);
const CLIENT: Token = Token(2);


pub fn hello_from_str<T>(
  ip_addr: &str,
  port: u16,
  gmt_in_hr: i32,
  callback: fn(TcpStream, u128) -> T,
) -> Result<(), Error> where
    T: Future + Send + 'static {
  match ip_addr.parse() {
    Ok(addr) => {
      hello(addr, port, gmt_in_hr, callback)
    }
    Err(err) => {
      panic!("Failed to parse IpAddr from [{}]! Err = [{}]", ip_addr, err)
    }
  }
}

pub fn hello_from_arr<T, U>(
  ip_addr: U,
  port: u16,
  gmt_in_hr: i32,
  callback: fn(TcpStream, u128) -> T,
) -> Result<(), Error> where
    T: Future + Send + 'static,
    U: Into<IpAddr> {
  hello(ip_addr.into(), port, gmt_in_hr, callback)
}


pub fn hello<T>(
  ip_addr: IpAddr,
  port: u16,
  gmt_in_hr: i32,
  callback: fn(TcpStream, u128) -> T,
) -> Result<(), Error> where
    T: Future + Send + 'static {
// Parse IP Address into Socket Address
  let ref socket_addr = SocketAddr::new(ip_addr.into(), port);

// Setup the server socket
  let server = TcpListener::bind(socket_addr)?;

// Create a POLL instance
  let poll: Poll = Poll::new()?;

// Start listening for incoming connections
  poll.register(&server, SERVER_ACCEPT, Ready::readable(),
                PollOpt::edge())?;

// Setup the client socket
  let mut client = TcpStream::connect(socket_addr)?;

  let mut server_handler = None;

// Register the client
  poll.register(&client, CLIENT, Ready::readable() | Ready::writable(),
                PollOpt::edge())?;

// Create storage for events
  let mut events = Events::with_capacity(1024);

  let start = Instant::now();
  let timeout = Duration::from_millis(10);
  'top: loop {
    poll.poll(&mut events, None)?;
    for event in events.iter() {
      if start.elapsed() >= timeout {
        break 'top;
      }
      match event.token() {
        SERVER_ACCEPT => {
          let (handler, addr) = server.accept()?;
          println!("accept from addr: {}", &addr);
          poll.register(&handler, SERVER,
                        Ready::readable() | Ready::writable(), PollOpt::edge())?;
          server_handler = Some(handler);
        }

        SERVER => {
          if event.readiness().is_writable() {
            if let Some(ref mut handler) = &mut server_handler {
              match handler.write(b"SERVER_HELLO") {
                Ok(_) => {
                  println!("server wrote");
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => continue,
                err => {
                  err?;
                }
              }
            }
          }
          if event.readiness().is_readable() {
            let mut hello = [0; 12];
            if let Some(ref mut handler) = &mut server_handler {
              match handler.read_exact(&mut hello) {
                Ok(_) => {
                  assert_eq!(b"CLIENT_HELLO", &hello);
                  println!("server received");
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => continue,
                err => {
                  err?;
                }
              }
            }
          }
        }

        CLIENT => {
          if event.readiness().is_writable() {
            match client.write(b"CLIENT_HELLO") {
              Ok(_) => {
                println!("client wrote");
              }
              Err(ref err) if err.kind() == ErrorKind::WouldBlock => continue,
              err => {
                err?;
              }
            }
          }
          if event.readiness().is_readable() {
            let mut hello = [0; 12];
            match client.read_exact(&mut hello) {
              Ok(_) => {
                assert_eq!(b"SERVER_HELLO", &hello);
                println!("client received");
              }
              Err(ref err) if err.kind() == ErrorKind::WouldBlock => continue,
              err => {
                err?;
              }
            }
          }
        }
        _ => unreachable!(),
      }
    }
  };
  Ok(())
}




/*use std::future::Future;

use async_std::{println, task};
use async_std::net::TcpListener;
pub use async_std::net::TcpStream;
use async_std::prelude::*;
use chrono::{FixedOffset, Utc};

pub mod http;

pub fn hello<T: Future + Send + 'static>(
  ip_addrs: [&str; 2],
  gmt_in_hr: i32,
  callback: fn(TcpStream, u128) -> T,
) {
  task::block_on(async move {
    let tcp_listeners: [TcpListener; 2] = [
      {
        let listener = TcpListener::bind(ip_addrs[0])
          .await.expect(format!("Failed to bind addr [{}]!", ip_addrs[0]).as_str());
        println!("Listening on  [{}]!", ip_addrs[0]).await;
        listener
      },
      {
        let listener = TcpListener::bind(ip_addrs[1])
          .await.expect(format!("Failed to bind addr [{}]!", ip_addrs[1]).as_str());
        println!("Listening on  [{}]!", ip_addrs[1]).await;
        listener
      }];

    let mut tcp_counter: u128 = 0;
    let mut incoming_tcps = tcp_listeners[0].incoming().merge(tcp_listeners[1].incoming());

    while let Some(stream) = incoming_tcps.next().await {
      let stream: TcpStream = stream
        .expect("Failed to get stream from incoming TCP connection!");

      tcp_counter = (tcp_counter + 1) % (std::u128::MAX - 1);
      const DASH: &str = "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}";
      let local_timezone_offset = FixedOffset::east_opt(gmt_in_hr * 3600)
        .expect(&format!("Failed to parse timezone UTC{}!", gmt_in_hr));

      println!("\u{250C}{}", DASH).await;
      println!("\u{2502} Incoming TCP #{} @ {:?} ", tcp_counter,
               Utc::now().with_timezone(&local_timezone_offset)).await;

      let peer_addr = stream.peer_addr().expect("Failed to get peer address of stream!");
      let local_addr = stream.local_addr().expect("Failed to get local address of stream!");
      println!("\u{2502} Accepting from [{}] to [{}]", peer_addr, local_addr).await;
      println!("\u{2514}{}", DASH).await;

      task::spawn(async move {
        callback(stream, tcp_counter).await;

        // TODO change to `async println!` when possible!
        std::println!("\u{250C}{}", DASH);
        //.await;
        std::println!("\u{2502} End of TCP connection #{} from [{}]", tcp_counter, peer_addr);
        //.await;
        std::println!("\u{2514}{}", DASH);//.await;
      });
    }
  });
}
*/