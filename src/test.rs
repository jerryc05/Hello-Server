use crate::create_server;

/// Default internal port
pub const DEFAULT_PORT: u16 = 6006;

/// Default listening ip
pub const DEFAULT_IP_ADDRESS: &str = "127.0.0.1";

//#[cfg(test)]
//mod test {
//  use crate::test::{DEFAULT_IP_ADDRESS, DEFAULT_PORT};

  #[test]
  fn test() {
    create_server(DEFAULT_IP_ADDRESS,DEFAULT_PORT,
    |r_stream, w_stream| {

    }).run();
  }
//}