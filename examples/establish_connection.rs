use async_std::net::TcpStream;

//  use async_std::println;

/// Default listening ip
const DEFAULT_IP_ADDRESS_1: &str = "127.0.0.1:6006";
const DEFAULT_IP_ADDRESS_2: &str = "::1:6006";

fn main() {
  hello_server::hello([DEFAULT_IP_ADDRESS_1, DEFAULT_IP_ADDRESS_2],
                      -5, process);
}

async fn process(_tcp_stream: TcpStream, _tcp_num: u128) {
  // TODO change to `async println!` when possible!
  std::println!("#{}: Simulate processing 1/5 tcp stream!", _tcp_num);
  //.await;
  std::println!("#{}: Simulate processing 2/5 tcp stream!", _tcp_num);
  //.await;
  std::println!("#{}: Simulate processing 3/5 tcp stream!", _tcp_num);
  //.await;
  std::println!("#{}: Simulate processing 4/5 tcp stream!", _tcp_num);
  //.await;
  std::println!("#{}: Simulate processing 5/5 tcp stream!", _tcp_num);
  //.await;
}