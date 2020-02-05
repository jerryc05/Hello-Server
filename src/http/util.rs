//use async_std::io::prelude::*;
use crate::TcpStream;

//use async_std::task;
//
pub fn parse_tcp(mut tcp_stream: TcpStream) -> String {
  unimplemented!()
//  const DEFAULT_BUFFER_SIZE: u8 = std::u8::MAX;
//  let buffer = &mut [0; DEFAULT_BUFFER_SIZE as usize];
//  let mut vec_buffer = vec![];
//
//  if vec_buffer.capacity() < DEFAULT_BUFFER_SIZE as usize {
//    vec_buffer.reserve(DEFAULT_BUFFER_SIZE as usize);
//  }
//  while let Ok(n) = task::block_on(tcp_stream.read(buffer)) {
//    if n > 0 {
//      std::println!("Read [{}] bytes!", n);
//      vec_buffer.extend_from_slice(&buffer[..n]);
//      if n == DEFAULT_BUFFER_SIZE as usize {
//        vec_buffer.reserve(vec_buffer.capacity() + DEFAULT_BUFFER_SIZE as usize);
//        continue;
//      }
//    }
//    break;
//  };
//  unsafe { String::from_utf8_unchecked(vec_buffer) }
}