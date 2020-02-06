use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, Hash, Hasher};

use mio::{Poll, Token};

pub struct TokenMgr(HashMap<usize, Token, BuildNoHashUsizeHasher>);

impl TokenMgr {
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
    self.0.remove(&token.0);
    poll.registry().deregister(token);
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
    self.0 = u64::from(i)
  }
}

impl Default for NoHashUsizeHasher {
  fn default() -> Self {
    NoHashUsizeHasher(1)
  }
}

type BuildNoHashUsizeHasher = BuildHasherDefault<NoHashUsizeHasher>;