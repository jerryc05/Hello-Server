use std::future::Future;

use async_std::{println, task};
use async_std::net::TcpListener;
pub use async_std::net::TcpStream;
use async_std::prelude::*;
use chrono::{FixedOffset, Utc};

pub mod http;

pub fn hello<T: Future + Send + 'static>(
  ip_addrs: [&str; 2],
  timezone_in_hr: i32,
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
      let local_timezone_offset = FixedOffset::east_opt(timezone_in_hr * 3600)
        .expect(&format!("Failed to parse timezone UTC{}!", timezone_in_hr));

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