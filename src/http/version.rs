/// Enum of Http Version field
#[derive(Debug)]
pub enum HttpVersion {
  // HTTP/0.9
  Http09,

  // HTTP/1.0
  Http10,

  // HTTP/1.1
  Http11,

  // HTTP/2.0
  Http20,
}

impl Into<HttpVersion> for &str {
  fn into(self) -> HttpVersion {
    match self {
      "HTTP/0.9" => HttpVersion::Http09,
      "HTTP/1.0" => HttpVersion::Http10,
      "HTTP/1.1" => HttpVersion::Http11,
      "HTTP/2.0" => HttpVersion::Http20,
      _ => panic!("Bad Http Version [{}]!", self)
    }
  }
}