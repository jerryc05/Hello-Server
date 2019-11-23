use std::any::Any;
use std::env;

use tokio;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use crate::http::request::HTTPRequest;

mod constants;
mod http;

#[tokio::main]
async fn main() {
  /* Allow passing a port number to listen on as the first argument of this
   * program.
   */
  let port = env::args().nth(1).map_or(
    constants::DEFAULT_PORT,
    |_port| _port.parse().expect(format!("Failed to parse port [{}] to int!", _port).as_str()));

  /* Next up we create a TCP listener which will listen for incoming
   * connections. This TCP listener is bound to the address we determined
   * above and must be associated with an event loop.
   */
  let mut listener = TcpListener::bind((constants::DEFAULT_IP_ADDRESS, port))
    .await.expect(format!("Failed to bind port [{}]!", port).as_str());
  println!("Listening on {}:{}", constants::DEFAULT_IP_ADDRESS, port);

  loop {
    // Asynchronously wait for an inbound socket. f:off
    let (mut tcp_stream, socket_address) = listener.accept().await
        .expect("Failed to accept new incoming TCP stream!"); // f:on

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

      let (mut read_stream, mut write_stream) = tcp_stream.split();
      let mut bytes_buffer = [0; 512];

      // In a loop, read data from the socket and write the data back.
      loop {
        let n = read_stream.read(&mut bytes_buffer)
                           .await.expect("Failed to read lines from socket!");
        if n > 0 {
          print!("{}", unsafe { std::str::from_utf8_unchecked(&bytes_buffer) });

          if n < bytes_buffer.len() {
          tcp_stream.write_all("HTTP/1.1 200 OK\r\n\r\nHello!".as_bytes())
                    .await.expect("Failed to write to TCP Stream!");
            println!("write finished!",);
            break;
          }
        }
      }
    });
  }
}
