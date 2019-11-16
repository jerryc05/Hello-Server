use std::env;
use std::error::Error;

use tokio;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

mod constants;
mod http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // Allow passing a port number to listen on as the first argument of this
  // program.
  let port = env::args().nth(1).map_or(
    constants::DEFAULT_PORT,
    |_port| _port.parse().expect(
      format!("Failed to parse port [{}] to int!", _port).as_str()));

  // Next up we create a TCP listener which will listen for incoming
  // connections. This TCP listener is bound to the address we determined
  // above and must be associated with an event loop.
  let mut listener = TcpListener::bind(("127.0.0.1", port))
    .await.expect(format!("Failed to bind port [{}]!", port).as_str());
  println!("Listening on: 127.0.0.1:{}", port);

  loop {
    // Asynchronously wait for an inbound socket.
    let (mut socket, _) = listener.accept().await?;

    // And this is where much of the magic of this server happens. We
    // crucially want all clients to make progress concurrently, rather than
    // blocking one on completion of another. To achieve this we use the
    // `tokio::spawn` function to execute the work in the background.
    //
    // Essentially here we're executing a new task to run concurrently,
    // which will allow all of our clients to be processed concurrently.
    tokio::spawn(async move {
      let mut raw_request = Vec::<u8>::new();
      let mut buffer = [0; 64];

      // In a loop, read data from the socket and write the data back.
      loop {
        let n = socket.read(&mut buffer)
                      .await
                      .expect("Failed to read data from socket");

        if n == 0 {
          break;
        } else {
          raw_request.extend(&buffer[..n]);
          println!("---------------- Read {:4} bytes! ----------------", n);
          println!("{}", unsafe { std::str::from_utf8_unchecked(&buffer) });
          println!("---------------- End {:4} bytes! -----------------", n);
        }
      }

//      let request =
//
//        socket.write_all(&buffer[0..n])
//              .await
//              .expect("Failed to write data to socket");
    });
  }
}
