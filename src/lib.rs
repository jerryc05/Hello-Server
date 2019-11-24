#![feature(async_await)]

use tokio;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::split::{ReadHalf, WriteHalf};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

use crate::http::request::HTTPRequest;

mod http;
mod test;

static RT: Runtime = Runtime::new();

struct HelloServer<'a> {
  ip_address: &'a str,
  port: u16,
  callback: fn(ReadHalf, WriteHalf) -> (),
}

pub fn create_server(ip_address: &str, port: u16,
                     callback: fn(ReadHalf, WriteHalf)) -> HelloServer {
  HelloServer { ip_address, port, callback }
}


impl<'a> HelloServer<'a> {
  pub fn rrun(&'a self) {
    RT.block_on(self.run())
  }

  pub async fn run(&'a self) {
    /* Next up we create a TCP listener which will listen for incoming
     * connections. This TCP listener is bound to the address we determined
     * above and must be associated with an event loop.
     */
    let mut listener = TcpListener::bind((self.ip_address, self.port))
      .await.expect(format!("Failed to bind port [{}]!", self.port).as_str());
    println!("Listening on {}:{}", self.ip_address, self.port);

    loop {
      // Asynchronously wait for an inbound socket. f:off
      let (mut tcp_stream, socket_address) = listener.accept()
          .await.expect("Failed to accept new incoming TCP stream!"); // f:on

      /* And this is where much of the magic of this server happens. We
       * crucially want all clients to make progress concurrently, rather than
       * blocking one on completion of another. To achieve this we use the
       * `tokio::spawn` function to execute the work in the background.
       *
       * Essentially here we're executing a new task to run concurrently,
       * which will allow all of our clients to be processed concurrently.
       */
      tokio::spawn(async move {
        println!("\n\n\n--- Incoming TCP connection from {:?} ---", socket_address);
        let (mut r_stream, mut w_stream) = tcp_stream.split();
        (self.callback)(r_stream, w_stream);
      });
    }
  }
}