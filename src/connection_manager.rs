use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use mio::{Poll, Token};
use mio::net::TcpListener;

type K = usize;
type V<'a> = &'a mut TcpListener;

/// A Manager that maps `Token ID` to `TcpListener`.
pub struct TokenMgr<'a>(HashMap<K, V<'a>, BuildNoHashUsizeHasher>);

impl TokenMgr<'_> {
  pub fn new() -> Self {
    let map = HashMap::with_hasher(
      BuildNoHashUsizeHasher::default());
    TokenMgr(map)
  }

  pub fn next_token(&self) -> Token {
    for i in (1..=usize::max_value() - 1).step_by(2) {
      if !self.0.contains_key(&i) {
        return Token(i);
      }
    }
    panic!("No more available tokens!")
  }

  pub fn release_token(&mut self, token: &mut Token, poll: &Poll) {
    match self.0.remove(&token.0) {
      Some(listener) => { poll.registry().deregister(listener); }
      _ => { panic!("Token [{}] already removed from map unexpectedly!", token.0); }
    };
  }
}

struct NoHashUsizeHasher(u64);

impl Hasher for NoHashUsizeHasher {
  fn finish(&self) -> u64 {
    self.0
  }

  fn write(&mut self, bytes: &[u8]) {
    unreachable!("Incorrect use of NoHash-Hasher!")
  }

  fn write_usize(&mut self, i: usize) {
    match u64::try_from(i) {
      Ok(value) => self.0 = value,
      Err(e) => panic!("Failed to parse [{}] from usize to u64! [{}]", i, e)
    }
  }
}

impl Default for NoHashUsizeHasher {
  fn default() -> Self {
    NoHashUsizeHasher(1)
  }
}

type BuildNoHashUsizeHasher = BuildHasherDefault<NoHashUsizeHasher>;