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

use std::convert::TryFrom;

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

impl<'a> TryFrom<&'a str> for HTTPRequest<'a> {
  type Error = &'static str;

  fn try_from(s: &'a str) -> Result<Self, Self::Error> {
    let mut method = None;
    let mut request_uri = None;
    let mut http_version = None;
    let mut header = None;
    let mut body = None;

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
    } else {
      Err("Invalid request content!")
    }
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
  Accept(Vec<HTTPRequestHeaderAccept<'a>>),
  AcceptEncoding(Vec<&'a str>),
  AcceptLanguage(&'a str),
  Connection(&'a str),
  ContentLength(usize),
  ContentType(&'a str),
  Host(&'a str),
  Referer(&'a str),
  UserAgent(&'a str),
  _OtherHeader(&'a str, &'a str),
}

/// Struct of Header field "Accept"
#[derive(Debug)]
pub struct HTTPRequestHeaderAccept<'a> {
  mime_type: &'a str,
  mime_subtype: &'a str,
  q_factor_weighting: Option<f32>,
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
      "ACCEPT" => {
        let mut vec = vec![];
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        for mut item in s.split(',') {
          item = item.trim();

          let slash_index = item.find('/')
              .expect("Failed to parse MIME Type in Accept header!");
          let mime_type = &item[..slash_index];
          item = &item[(slash_index + 1)..];

          let semicolon_index = item.find(';');
          let mime_subtype;
          let q_factor_weighting;

          if semicolon_index.is_some() {
            let semicolon_index = semicolon_index
                .expect("Failed to parse MIME Subtype in Accept header!");
            mime_subtype = &item[..semicolon_index];
            item = &item[(semicolon_index + 1)..];
            while item.starts_with(' ') || item.starts_with('q') ||
                item.starts_with('=') { item = &item[1..]; }

            q_factor_weighting = Some(item.parse::<f32>()
                .expect("Failed to parse Q-Factor Weighting to float number"));
          } else {
            mime_subtype = item;
            q_factor_weighting = None;
          }
          vec.push(HTTPRequestHeaderAccept { mime_type, mime_subtype, q_factor_weighting });
        }
        HTTPRequestHeader::Accept(vec)
      }
      "ACCEPT-LANGUAGE" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::AcceptLanguage(s)
      }
      "ACCEPT-ENCODING" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::AcceptEncoding(s.split_ascii_whitespace().collect())
      }
      "CONNECTION" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::Connection(s)
      }
      "CONTENT-LENGTH" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::ContentLength(s.parse()
            .expect("Failed to convert Content-Length into usize!"))
      }
      "CONTENT-TYPE" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::ContentType(s)
      }
      "HOST" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::Host(s)
      }
      "REFERER" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::Referer(s)
      }
      "USER-AGENT" => {
        let mut s = &s[colon + 1..];
        while s.starts_with(' ') {
          s = &s[1..];
        }
        HTTPRequestHeader::UserAgent(s)
      }
      _ => {
        HTTPRequestHeader::_OtherHeader(&s[..colon],
                                        &s[colon + 2..])
      }
    }
  }
}