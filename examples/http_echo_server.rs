#![feature(async_closure)]

use async_std::io::*;
use async_std::io::prelude::*;
use async_std::net::TcpStream;

//  use async_std::println;

/// Default listening ip
const DEFAULT_IP_ADDRESS_1: &str = "127.0.0.1:6006";
const DEFAULT_IP_ADDRESS_2: &str = "::1:6006";

fn main() {
  hello_server::hello([DEFAULT_IP_ADDRESS_1, DEFAULT_IP_ADDRESS_2],
                      -5,
                      async move |_tcp_stream: TcpStream, _tcp_num: u128| {
//                        let mut lines = BufReader::new(&_tcp_stream).lines();
//                        while let Some(line) = lines.next().await {

//        let line = line?;
//        let (dest, msg) = match line.find(':') {
//          None => continue,
//          Some(idx) => (&line[..idx], line[idx + 1..].trim()),
//        };
//        let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
//        let msg: String = msg.to_string();
//
//        broker.send(Event::Message { // 4
//          from: name.clone(),
//          to: dest,
//          msg,
//        }).await.unwrap();
//                        }
                      });
}