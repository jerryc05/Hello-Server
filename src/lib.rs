use async_std::{println, task};
use async_std::net::{TcpListener, TcpStream};
use chrono::{FixedOffset, Utc};
use futures::StreamExt;

mod http;

pub struct HelloServer {
  ip_addr: &'static str,
  port: u16,
  local_timezone_offset: FixedOffset,
  callback: fn(TcpStream, &u128),
}

impl HelloServer {
  pub fn new(ip_addr: &'static str, port: u16, timezone_in_hr: i32,
             callback: fn(TcpStream, &u128)) -> HelloServer {
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

          println!("\u{250C}{}", DASH).await;
          tcp_counter = (tcp_counter + 1) % (std::u128::MAX - 1);
          let tcp_num: u128 = tcp_counter;
          println!("\u{2502} Incoming TCP #{} @ {:?} ", tcp_num,
                   Utc::now().with_timezone(&self.local_timezone_offset)).await;

          let peer_addr = stream.peer_addr().expect("Failed to get peer address of stream!");
          println!("\u{2502} Accepting from [{}] to [{}]",
                   peer_addr,
                   stream.local_addr().expect("Failed to get local address of stream!")).await;
          println!("\u{2514}{}", DASH).await;

          let callback = self.callback;
          task::spawn(async move {
            callback(stream, &tcp_num);
            std::println!("\u{250C}{}", DASH);
            std::println!("\u{2502} End of TCP connection #{} from [{}]", tcp_num, peer_addr);
            std::println!("\u{2514}{}", DASH);
          });
        }
      }
    });
  }
}