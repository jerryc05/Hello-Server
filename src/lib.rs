use async_std::{println, task};
use async_std::net::{TcpListener, TcpStream};
use chrono::{FixedOffset, Utc};
use futures::StreamExt;

mod http;

pub struct HelloServer {
  ip_addr: &'static str,
  port: u16,
  local_timezone_offset: FixedOffset,
  callback: fn(TcpStream),
}

impl HelloServer {
  pub fn new(ip_addr: &'static str, port: u16, timezone_in_hr: i32,
             callback: fn(TcpStream)) -> HelloServer {
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

      loop {
        while let Some(stream) = incoming_tcp.next().await {
          let stream: TcpStream = stream
            .expect("Failed to get stream from incoming TCP connection!");

          println!("\u{250C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}").await;
          println!("\u{2502} Incoming TCP @ {:?} ",
                   Utc::now().with_timezone(&self.local_timezone_offset)).await;
          println!("\u{2502} Accepting from [{}] to [{}]",
                   stream.peer_addr().expect("Failed to get peer address of stream!"),
                   stream.local_addr().expect("Failed to get local address of stream!")).await;
          println!("\u{251C}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}").await;

          let callback = self.callback;
          task::spawn(async move {
            callback(stream);
            std::println!("\u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}");
          });
        }
      }
    });
  }
}