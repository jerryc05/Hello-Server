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

impl<'a> From<&'a str> for HTTPRequest<'a> {
  fn from(s: &'a str) -> HTTPRequest<'a> {
    let mut request = HTTPRequest {
      method: HttpMethod::Get,
      request_uri: RequestURI::Asterisk,
      http_version: HttpVersion::Http_1_1,
      header: Vec::new(),
      body: String::new(),
    };
    let mut status = ProcessingRequestLine;

    for line in s.split("\r\n") {
      match status {
        ProcessingRequestLine => {
          let req_line = line.split(' ').collect::<Vec<&str>>();
          println!("req-line=[{:?}]",req_line);
          assert!(req_line.len() >= 3);

          request.method = req_line[0].into();
          request.request_uri = req_line[1].into();
          request.http_version = req_line[2].into();
          status = ProcessingHeaders;
        }

        ProcessingHeaders => {
          if !line.is_empty() {
            request.header.push(line.into())
          } else {
            status = ProcessingBodyFirstLine;
            request.body = String::new();
          }
        }

        ProcessingBodyFirstLine => {
          request.body.push_str(line);
          status = ProcessingBody;
        }

        ProcessingBody => {
          request.body.push_str("\r\n");
          request.body.push_str(line);
        }
      }
    };
    request
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
  ProcessingBodyFirstLine,
  ProcessingBody,
}

impl From<&str> for HttpMethod {
  fn from(s: &str) -> HttpMethod {
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
  fn from(s: &'a str) -> RequestURI<'a> {
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
  fn from(s: &'a str) -> HTTPRequestHeader<'a> {
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