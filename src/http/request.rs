//! Sample HTTP Request Format
//! ```no run
//! A Request-line (Method SP Request-URI SP HTTP-Version CRLF)
//!
//! Zero or more header (General|Request|Entity) fields followed by CRLF
//!
//! An empty line (i.e., a line with nothing preceding the CRLF)
//! indicating the end of the header fields
//!
//! Optionally a message-body
//! ```
//!
//! Example:
//! ```no run
//! POST /cgi-bin/process.cgi HTTP/1.1
//! User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
//! Host: www.example.com
//! Content-Type: application/x-www-form-urlencoded
//! Content-Length: length
//! Accept-Language: en-us
//! Accept-Encoding: gzip, deflate
//! Connection: Keep-Alive
//!
//! licenseID=string&content=string&/paramsXML=string
//! ```

use async_std::io::prelude::*;
use async_std::net::TcpStream;
use async_std::task;

use crate::http::request::HTTPRequestParsingState::*;
use crate::http::version::HttpVersion;

/// Struct of parsed HTTP Request
#[derive(Debug)]
pub struct HTTPRequest<'a> {
  // Request line
  pub method: HttpMethod,
  pub request_uri: RequestURI<'a>,
  pub http_version: HttpVersion,

  // Header fields
  pub header: Vec<HTTPRequestHeader<'a>>,

  // Body field
  pub body: String,
}

impl<'a> HTTPRequest<'a> {
  fn from_str(s: &'a str) -> Result<Self, ()> {
    let mut method: Option<HttpMethod> = None;
    let mut request_uri: Option<RequestURI> = None;
    let mut http_version: Option<HttpVersion> = None;
    let mut header: Option<Vec<HTTPRequestHeader<'a>>> = None;
    let mut body: Option<String> = None;

    let mut status = ProcessingRequestLine;
    let mut parsed_str_count: usize = 0;

    for line in s.split("\r\n") {
      match status {
        ProcessingRequestLine => {
          let req_line = line.split(' ').collect::<Vec<&str>>();
          if req_line.len() < 3 {
            println!("Failed to parse Request Line from [{:?}]", line);
          } else {
            method = Some(req_line[0].into());
            request_uri = Some(req_line[1].into());
            http_version = Some(req_line[2].into());
            status = ProcessingHeaders;
          }
          parsed_str_count += line.len() + 2;
        }

        ProcessingHeaders => {
          if !line.is_empty() {
            if header.is_none() {
              header = Some(vec![])
            }
            header.as_mut().expect("Failed to get header vector!")
                  .push(line.into())
          } else {
            status = ProcessingBody;
          }
          parsed_str_count += line.len() + 2;
        }

        ProcessingBody => {
          body = Some((&s[parsed_str_count..]).to_owned());
        }
      }
    };
    if method.is_some() && request_uri.is_some() && http_version.is_some() {
      Ok(HTTPRequest {
        method: method.expect("Failed to get METHOD!"),
        request_uri: request_uri.expect("Failed to get REQUEST URI!"),
        http_version: http_version.expect("Failed to get HTTP VERSION!"),
        header: header.unwrap_or_default(),
        body: body.unwrap_or_default(),
      })
    } else { Err(()) }
  }

  pub fn from_stream_with_buffer(
    mut tcp_stream: TcpStream,
    vec_buffer: &'a mut Vec<u8>,
  ) -> Result<Self, ()> {
    const DEFAULT_BUFFER_SIZE: u8 = std::u8::MAX;
    let buffer = &mut [0; DEFAULT_BUFFER_SIZE as usize];

    if vec_buffer.capacity() < DEFAULT_BUFFER_SIZE as usize {
      vec_buffer.reserve(DEFAULT_BUFFER_SIZE as usize);
    }
    while let Ok(n) = task::block_on(tcp_stream.read(buffer)) {
      if n > 0 {
        std::println!("Read [{}] bytes!", n);
        vec_buffer.extend_from_slice(&buffer[..n]);
        if n == DEFAULT_BUFFER_SIZE as usize {
          vec_buffer.reserve(vec_buffer.capacity() + DEFAULT_BUFFER_SIZE as usize);
          continue;
        }
      }
      break;
    };
    HTTPRequest::from_str(std::str::from_utf8(vec_buffer)
      .expect("Failed to parse UTF-8 str from buffer!"))
  }
}

/// Enum of HTTP Method field
#[derive(Debug)]
pub enum HttpMethod {
  /* The GET method is used to retrieve information from the given server using
   * a given URI. Requests using GET should only retrieve data and should have
   * no other effect on the data.
   */
  Get,

  // Same as GET, but it transfers the status line and the header section only.
  Head,

  /* A POST request is used to send data to the server, for example, customer
   * information, file upload, etc. using HTML forms.
   */
  Post,

  /* Replaces all the current representations of the target resource with the
   * uploaded content.
   */
  Put,

  // Removes all the current representations of the target resource given by URI.
  Delete,

  // Establishes a tunnel to the server identified by a given URI.
  Connect,

  // Describe the communication options for the target resource.
  Option,

  // Performs a message loop back test along with the path to the target resource.
  Trace,
}

/// Enum of Request URI field
#[derive(Debug)]
pub enum RequestURI<'a> {
  /* The asterisk * is used when an HTTP request does not apply to a particular
   * resource, but to the server itself, and is only allowed when the method
   * used does not necessarily apply to a resource. For example:
   * `OPTIONS * HTTP/1.1`
   */
  Asterisk,

  /* The absoluteURI is used when an HTTP request is being made to a proxy. The
   * proxy is requested to forward the request or service from a valid cache,
   * and return the response.  For example:
   * `GET http: *www.w3.org/pub/WWW/TheProject.html HTTP/1.1`
   */
  AbsoluteUri(&'a str),

  /* The most common form of Request-URI is that used to identify a resource on
   * an origin server or gateway. For example, a client wishing to retrieve a
   * resource directly from the origin server would create a TCP connection to
   * port 80 of the host "www.w3.org" and send the following lines:
   * ```
   * GET /pub/WWW/TheProject.html HTTP/1.1
   * Host: www.w3.org
   * ```
   * Note that the absolute path cannot be empty; if none is present in the
   * original URI, it MUST be given as "/" (the server root).
   */
  AbsolutePath(&'a str),
}

/// Enum of Header field
#[derive(Debug)]
pub enum HTTPRequestHeader<'a> {
  Accept(&'a str),
  AcceptEncoding(&'a str),
  AcceptLanguage(&'a str),
  Connection(&'a str),
  ContentLength(usize),
  ContentType(&'a str),
  Host(&'a str),
  Referer(&'a str),
  UserAgent(&'a str),
  _OtherHeader(&'a str, &'a str),
}

/// Enum of states when parsing from str/String to HTTPRequest
#[derive(Debug)]
enum HTTPRequestParsingState {
  ProcessingRequestLine,
  ProcessingHeaders,
  ProcessingBody,
}

impl From<&str> for HttpMethod {
  fn from(s: &str) -> Self {
    match s.to_ascii_uppercase().as_str() {
      "GET" => HttpMethod::Get,
      "HEAD" => HttpMethod::Head,
      "POST" => HttpMethod::Post,
      "PUT" => HttpMethod::Put,
      "DELETE" => HttpMethod::Delete,
      "CONNECT" => HttpMethod::Connect,
      "OPTION" => HttpMethod::Option,
      "TRACE" => HttpMethod::Trace,
      _ => panic!("Bad HTTP Method [{}]!", s)
    }
  }
}

impl<'a> From<&'a str> for RequestURI<'a> {
  fn from(s: &'a str) -> Self {
    match s {
      "*" => RequestURI::Asterisk,
      _ => {
        if s.contains("://") {
          RequestURI::AbsoluteUri(s)
        } else if s.starts_with('/') {
          RequestURI::AbsolutePath(s)
        } else {
          panic!("Bad Request URI [{}]!", s)
        }
      }
    }
  }
}

impl<'a> From<&'a str> for HTTPRequestHeader<'a> {
  fn from(s: &'a str) -> Self {
    let colon = s.find(':').expect(
      format!("Failed to find a colon in [{}]!", s).as_str()
    );

    match s[..colon].to_ascii_uppercase().as_str() {
      "ACCEPT" => HTTPRequestHeader::Accept(&s[colon + 2..]),
      "ACCEPT-LANGUAGE" => HTTPRequestHeader::AcceptLanguage(&s[colon + 2..]),
      "ACCEPT-ENCODING" => HTTPRequestHeader::AcceptEncoding(&s[colon + 2..]),
      "CONNECTION" => HTTPRequestHeader::Connection(&s[colon + 2..]),
      "CONTENT-LENGTH" => HTTPRequestHeader::ContentLength((&s[colon + 2..]).parse()
                                                                            .expect("Failed to convert Content-Length into usize!")),
      "CONTENT-TYPE" => HTTPRequestHeader::ContentType(&s[colon + 2..]),
      "HOST" => HTTPRequestHeader::Host(&s[colon + 2..]),
      "REFERER" => HTTPRequestHeader::Referer(&s[colon + 2..]),
      "USER-AGENT" => HTTPRequestHeader::UserAgent(&s[colon + 2..]),
      _ => {
        HTTPRequestHeader::_OtherHeader(&s[..colon],
                                        &s[colon + 2..])
      }
    }
  }
}