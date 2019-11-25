use async_std::{println, task};
use async_std::future::Future;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use chrono::{FixedOffset, Utc};

mod http;

pub struct HelloServer<T: Future + 'static + Send> {
  ip_addr: &'static str,
  port: u16,
  local_timezone_offset: FixedOffset,
  callback: fn(TcpStream, u128) -> T,
}

impl<T: Future + Send + 'static> HelloServer<T> {
  pub fn new(ip_addr: &'static str,
             port: u16,
             timezone_in_hr: i32,
             callback: fn(TcpStream, u128) -> T,
  ) -> HelloServer<T> {
    HelloServer {
      ip_addr,
      port,
      local_timezone_offset: FixedOffset::east_opt(timezone_in_hr * 3600)
        .expect(&format!("Failed to parse timezone UTC{}!", timezone_in_hr)),
      callback,
    }
  }

  pub fn run(self) {
    task::block_on(async move {
      let listener: TcpListener = TcpListener::bind((self.ip_addr, self.port))
        .await.expect(format!("Failed to bind port [{}]!", self.port).as_str());

      println!("Listening on  [{}:{}]!", self.ip_addr, self.port).await;

      let mut incoming_tcp = listener.incoming();
      let mut tcp_counter: u128 = 0;

      loop {
        while let Some(stream) = incoming_tcp.next().await {
          let stream: TcpStream = stream
            .expect("Failed to get stream from incoming TCP connection!");

          const DASH: &str = "\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}";
          tcp_counter = (tcp_counter + 1) % (std::u128::MAX - 1);

          println!("\u{250C}{}", DASH).await;
          println!("\u{2502} Incoming TCP #{} @ {:?} ", tcp_counter,
                   Utc::now().with_timezone(&self.local_timezone_offset)).await;

          let peer_addr = stream.peer_addr().expect("Failed to get peer address of stream!");
          let local_addr = stream.local_addr().expect("Failed to get local address of stream!");
          println!("\u{2502} Accepting from [{}] to [{}]", peer_addr, local_addr).await;
          println!("\u{2514}{}", DASH).await;

//          let callback_func = self.callback;
          task::spawn(async move {
//            callback_func(stream, tcp_counter).await;
            println!("\u{250C}{}", DASH).await;
            println!("\u{2502} End of TCP connection #{} from [{}]", tcp_counter, peer_addr).await;
            println!("\u{2514}{}", DASH).await;
          });
        }
      }
    });
  }
}