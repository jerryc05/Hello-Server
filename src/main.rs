use hello_server::HelloServer;

/// Default internal port
const DEFAULT_PORT: u16 = 6006;

/// Default listening ip
const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";

fn main() {
  HelloServer::new(
    DEFAULT_IP_ADDRESS, DEFAULT_PORT,
    move |tcp_stream| {
      println!("Accepted [{:?}] from closure!", tcp_stream);
    }).run();
}