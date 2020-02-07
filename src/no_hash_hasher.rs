use std::convert::TryFrom;
use std::hash::{BuildHasherDefault, Hasher};

pub(crate) type BuildNoHashUsizeHasher = BuildHasherDefault<NoHashUsizeHasher>;

/// A hasher that does literally no hashing computation at all.
pub(crate) struct NoHashUsizeHasher(u64);

impl Hasher for NoHashUsizeHasher {
  fn finish(&self) -> u64 {
    self.0
  }

  fn write(&mut self, _bytes: &[u8]) {
    panic!("Incorrect use of NoHash-Hasher!")
  }

  fn write_u8(&mut self, i: u8) {
    self.0 = u64::from(i);
  }

  fn write_u16(&mut self, i: u16) {
    self.0 = u64::from(i);
  }

  fn write_u32(&mut self, i: u32) {
    self.0 = u64::from(i);
  }

  fn write_u64(&mut self, i: u64) {
    self.0 = u64::from(i);
  }

  fn write_usize(&mut self, i: usize) {
    match u64::try_from(i) {
      Ok(value) => self.0 = value,
      Err(err) => panic!("Failed to parse [{}] to u64! [{:?}]", i, err)
    }
  }
}

impl Default for NoHashUsizeHasher {
  fn default() -> Self {
    NoHashUsizeHasher(1)
  }
}