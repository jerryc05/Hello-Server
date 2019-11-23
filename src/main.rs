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
    let (mut tcp_stream, _) = listener.accept().await
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
      let (mut read_stream, mut write_stream) = tcp_stream.split();
      let mut bytes_buffer = [0; 128];
      let mut responded_http_version = false;
      let mut http_version_write_all = None;
//      let mut content_length = 0;

      println!("[Incoming connection!] --- --- ---");

      // In a loop, read data from the socket and write the data back.
      loop {
        let n = read_stream.read(&mut bytes_buffer)
                           .await.expect("Failed to read lines from socket!");
        if n > 0 {
          if !responded_http_version {
            responded_http_version = true;
            http_version_write_all = Some(write_stream.write_all("HTTP/1.1".as_bytes()));
          }
          print!("{}", unsafe { std::str::from_utf8_unchecked(&bytes_buffer) });

          if n < bytes_buffer.len() {
            http_version_write_all.expect("Fail to get initialized instance of http_version_write_all!")
              .await.expect("Failed to write HTTP Version!");
//          tcp_stream.write_all(" 200 OK\r\n\r\nHello!".as_bytes())
//                    .await.expect("Failed to write to TCP Stream!");
            println!("write finished!");
            break;
          }
        }
//        if n <= bytes_buffer.len() { break; }

//        let line_buffer = complete_request.lines().last()
//                                          .expect("Failed to get last line of complete request!");
//
//        if line_buffer.to_ascii_uppercase().contains("CONTENT-LENGTH") {
//          content_length = line_buffer[16..]
//            .parse().expect("Failed to parse Content-Length into usize while streaming!");
//          //
//          // Begin parse body
//        } else if line_buffer.is_empty() {
//          let mut content_buffer = Vec::with_capacity(content_length);
//
//          buffered_reader.read_exact(&mut content_buffer).await
//                         .expect("Failed to read content from socket!");
//          println!("---\ncontent_buffer [{}]", unsafe {
//            String::from_utf8_unchecked(content_buffer)
//          });
////          complete_request.push_str(
////            std::str::from_utf8(&content_buffer)
////              .expect("Failed to parse content to utf8!")
////          );
//          break;
//        }
      }

//      let http_request = HTTPRequest::from(complete_request.as_str());
//      println!("{:?}\n---", http_request)

//      let request =
//        socket.write_all(&buffer[0..n])
//              .await
//              .expe


//      let (read_end, mut write_end) = tcp_stream.split();
//      let mut buffered_reader = BufReader::new(read_end);
//      let mut complete_request = String::new();
//      let mut content_length = 0;
//
//      // In a loop, read data from the socket and write the data back.
//
//
//      // todo read until it gets "\r\n\r\n" and then parse content length, then read_exact until end.
//
//
//      loop {
//        let n = buffered_reader
//          .read_line(&mut complete_request).await
//          .expect("Failed to read lines from socket!");
//        if n <= 0 { break; }
//
//        let line_buffer = complete_request.lines().last()
//                                          .expect("Failed to get last line of complete request!");
//
//        if line_buffer.to_ascii_uppercase().contains("CONTENT-LENGTH") {
//          content_length = line_buffer[16..]
//            .parse().expect("Failed to parse Content-Length into usize while streaming!");
//          //
//          // Begin parse body
//        } else if line_buffer.is_empty() {
//          let mut content_buffer = Vec::with_capacity(content_length);
//
//          buffered_reader.read_exact(&mut content_buffer).await
//                         .expect("Failed to read content from socket!");
//          println!("---\ncontent_buffer [{}]", unsafe {
//            String::from_utf8_unchecked(content_buffer)
//          });
////          complete_request.push_str(
////            std::str::from_utf8(&content_buffer)
////              .expect("Failed to parse content to utf8!")
////          );
//          break;
//        }
//      }
//
//      let http_request = HTTPRequest::from(complete_request.as_str());
//      println!("{:?}\n---", http_request)
//
////      let request =
////        socket.write_all(&buffer[0..n])
////              .await
////              .expect("Failed to write data to socket");
    });
  }
}
