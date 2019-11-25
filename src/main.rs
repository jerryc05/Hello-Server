use hello_server::HelloServer;

/// Default internal port
const DEFAULT_PORT: u16 = 6006;

/// Default listening ip
const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";

fn main() {
  example_server()
}

fn example_server() {
  HelloServer::new(
    DEFAULT_IP_ADDRESS, DEFAULT_PORT, -5,
    move |tcp_stream, tcp_num| {
      println!("Simulate processing 1/5 tcp stream #{}!", tcp_num);
      std::thread::sleep(std::time::Duration::from_secs(1));
      println!("Simulate processing 2/5 tcp stream #{}!", tcp_num);
      std::thread::sleep(std::time::Duration::from_secs(1));
      println!("Simulate processing 3/5 tcp stream #{}!", tcp_num);
      std::thread::sleep(std::time::Duration::from_secs(1));
      println!("Simulate processing 4/5 tcp stream #{}!", tcp_num);
      std::thread::sleep(std::time::Duration::from_secs(1));
      println!("Simulate processing 5/5 tcp stream #{}!", tcp_num);
      std::thread::sleep(std::time::Duration::from_secs(1));
    }).run();
}