use std::collections::HashMap;
use std::io::Error;

use crate::{Poll, TcpStream, Token};
use crate::no_hash_hasher::BuildNoHashUsizeHasher;

type V = TcpStream;

/// A Map that maps `primitive type` to `object`.
pub(crate) struct ConnMgr(
  HashMap<usize, V, BuildNoHashUsizeHasher>);

impl ConnMgr {
  pub fn new() -> Self {
    let map =
      HashMap::with_hasher(BuildNoHashUsizeHasher::default());
    ConnMgr(map)
  }

  pub fn generate_token(&mut self, value: V) -> Token {
    for i in 1..=usize::max_value() {
      if !self.0.contains_key(&i) {
        self.0.insert(i, value);
        return Token(i);
      }
    }
    panic!("No more available tokens!")
  }

  pub fn get_stream(&mut self, token_id: &usize) -> Option<&mut V> {
    self.0.get_mut(token_id)
  }

  pub fn release_token(&mut self, token: &mut Token, poll: &Poll) -> Result<(), Error> {
    match self.0.remove(&token.0) {
      Some(mut listener) =>
        poll.registry().deregister(&mut listener),
      _ =>
        panic!("Token [{}] already removed from map unexpectedly!", token.0)
    }
  }
}