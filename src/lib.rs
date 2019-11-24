use async_std::{println, task};
use async_std::net::{TcpListener, TcpStream};
use futures::StreamExt;

mod http;

pub struct HelloServer {
  ip_addr: &'static str,
  port: u16,
  callback: fn(TcpStream),
}

impl HelloServer {
  pub fn new(ip_addr: &'static str, port: u16,
             callback: fn(TcpStream)) -> HelloServer {
    HelloServer { ip_addr, port, callback }
  }

  pub fn run(self) {
    task::block_on(async move {
      let listener: TcpListener = TcpListener::bind((self.ip_addr, self.port))
        .await.expect(format!("Failed to bind port [{}]!", self.port).as_str());

      println!("Listening on  [{}:{}]!", self.ip_addr, self.port).await;

      let mut incoming_tcp = listener.incoming();

      loop {
        while let Some(stream) = incoming_tcp.next().await {
          let callback = self.callback;
          let stream: TcpStream = stream
            .expect("Failed to get stream from incoming TCP connection!");
          println!("\n\n---\nAccepting from [{:?}]!", stream).await;

          task::spawn(async move {
            callback(stream);
          });
        }
      }
    });
  }
}