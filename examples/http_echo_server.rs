//use std::convert::TryFrom;

use std::net::Ipv4Addr;

//use hello_server::http::request::*;
//use async_std::io::*;
//use async_std::io::prelude::*;
//use async_std::println;
//use hello_server::http::util::*;
use hello_server::TcpStream;

/// Default listening ip
//const DEFAULT_IP_ADDRESS_1: &str = "127.0.0.1";
//const DEFAULT_IP_ADDRESS_2: &str = "::1";
const PORT: u16 = 6006;

fn main()  {
  hello_server::hello_from_into_addr(Ipv4Addr::LOCALHOST, PORT,
                               -5, process).unwrap()
}

async fn process(_tcp_stream: TcpStream, _tcp_num: u128) {
//  // TODO change to `async println!` when possible!
//  std::println!("{:?}", HTTPRequest::try_from(
//    parse_tcp(_tcp_stream).as_str()
//  ).unwrap());
}