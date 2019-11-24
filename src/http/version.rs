/// Enum of Http Version field
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum HttpVersion {
  // HTTP/0.9
  Http_0_9,

  // HTTP/1.0
  Http_1_0,

  // HTTP/1.1
  Http_1_1,

  // HTTP/2.0
  Http_2_0,
}

impl From<&str> for HttpVersion {
  fn from(s: &str) -> Self {
    match s {
      "HTTP/0.9" => HttpVersion::Http_0_9,
      "HTTP/1.0" => HttpVersion::Http_1_0,
      "HTTP/1.1" => HttpVersion::Http_1_1,
      "HTTP/2.0" => HttpVersion::Http_2_0,
      _ => panic!("Bad Http Version [{}]!", s)
    }
  }
}