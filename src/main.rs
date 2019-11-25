#![feature(async_closure)]

use hello_server::HelloServer;

/// Default internal port
const DEFAULT_PORT: u16 = 6006;

/// Default listening ip
const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";

fn main() {
  _establish_connection_example();
//  _echo_server_example();
}

fn _establish_connection_example() {
  use async_std::task::sleep;
  use std::time::Duration;
  use async_std::println;

  HelloServer::new(
    DEFAULT_IP_ADDRESS, DEFAULT_PORT, -5,
    async move |_tcp_stream, _tcp_num| {
      println!("#{}: Simulate processing 1/5 tcp stream!", _tcp_num).await;
      sleep(Duration::from_secs(1)).await;
      println!("#{}: Simulate processing 2/5 tcp stream!", _tcp_num).await;
      sleep(Duration::from_secs(1)).await;
      println!("#{}: Simulate processing 3/5 tcp stream!", _tcp_num).await;
      sleep(Duration::from_secs(1)).await;
      println!("#{}: Simulate processing 4/5 tcp stream!", _tcp_num).await;
      sleep(Duration::from_secs(1)).await;
      println!("#{}: Simulate processing 5/5 tcp stream!", _tcp_num).await;
      sleep(Duration::from_secs(1)).await;
    }).run();
}

fn _echo_server_example() {
  use async_std::{
    io::*,
    prelude::*,
  };

  HelloServer::new(
    DEFAULT_IP_ADDRESS, DEFAULT_PORT, -5,
    async move |_tcp_stream, _tcp_num| {
      let mut lines = BufReader::new(_tcp_stream).lines();
      while let Some(line) = lines.next().await {
        println!("{}", line.expect("Failed to get line from lines.next()!"))
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
      }
    }).run();
}