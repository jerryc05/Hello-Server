use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault};

use mio::{Poll, Token};

pub struct TokenMgr(HashMap<usize, Token>);

impl TokenMgr {
  pub fn new() -> TokenMgr {
    let x: dyn BuildHasher = BuildHasherDefault::default();
    TokenMgr(HashMap::with_hasher(x))
  }

  pub fn next_server_token(&self) -> Token {
    for i in (1..=usize::max_value() - 1).step_by(2) {
      if !self.contains_key(&i) {
        Token(i)
      }
    }
    panic!("No more available server tokens!")
  }

  pub fn next_client_token(&self, server_token: Token) -> Token {
    debug_assert!(!self.contains_key(&(server_token.0 + 1)));
    Token(server_token.0 + 1)
  }

  pub fn release_token(&mut self, token: Token, &mut poll: Poll) {
    let x = if token.0 % 2 == 0 { token.0 - 1 } else { token.0 };
    poll.registry().deregister(self.remove(&x));
    poll.registry().deregister(self.remove(&(x + 1)));
  }
}