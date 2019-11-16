//! Sample HTTP Request Format
//! ```
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
//! ```
//! POST /cgi-bin/process.cgi HTTP/1.1
//! User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)
//! Host: www.tutorialspoint.com
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
struct HTTPRequest<'a> {
  // Request line
  method: HttpMethod,
  request_uri: RequestURI<'a>,
  http_version: HttpVersion,

  // Header fields
  header: Vec<HttpMethod>,

  // Body field
  body: String,
}

impl<'a> Into<HTTPRequest<'a>> for &'a str {
  fn into(self) -> HTTPRequest<'a> {
    let mut request = HTTPRequest {
      method: HttpMethod::Get,
      request_uri: RequestURI::Asterisk,
      http_version: HttpVersion::Http11,
      header: Vec::new(),
      body: String::new(),
    };
    let mut status = ProcessingRequestLine;

    for line in self.split("\r\n") {
      match status {
        ProcessingRequestLine => {
          let req_line = line.split(' ').collect::<Vec<&str>>();
          assert!(req_line.len() >= 3);

          request.method = req_line[0].into();
          request.request_uri = req_line[1].into();
          request.http_version = req_line[2].into();
          status = ProcessingHeaders;
        }

        ProcessingHeaders => {
          if self.is_empty() {
            status = ProcessingBodyFirstLine;
            request.body = String::new();
            continue;
          }
          request.header.push(self.into())
        }

        ProcessingBodyFirstLine => {
          request.body.push_str(self);
          status = ProcessingBody;
        }

        ProcessingBody => {
          request.body.push_str("\r\n");
          request.body.push_str(self);
        }
      }
    };
    request
  }
}

impl<'a> Into<HTTPRequest<'a>> for &'a [u8] {
  fn into(self) -> HTTPRequest<'a> {
    unsafe {
      std::str::from_utf8_unchecked(self)
    }.into()
  }
}

/// Enum of HTTP Method field
#[derive(Debug)]
enum HttpMethod {
  // The GET method is used to retrieve information from the given server using
  // a given URI. Requests using GET should only retrieve data and should have
  // no other effect on the data.
  Get,

  // Same as GET, but it transfers the status line and the header section only.
  Head,

  // A POST request is used to send data to the server, for example, customer
  // information, file upload, etc. using HTML forms.
  Post,

  // Replaces all the current representations of the target resource with the
  // uploaded content.
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
enum RequestURI<'a> {
  // The asterisk * is used when an HTTP request does not apply to a particular
  // resource, but to the server itself, and is only allowed when the method
  // used does not necessarily apply to a resource. For example:
  // `OPTIONS * HTTP/1.1`
  Asterisk,

  // The absoluteURI is used when an HTTP request is being made to a proxy. The
  // proxy is requested to forward the request or service from a valid cache,
  // and return the response.  For example:
  // `GET http://www.w3.org/pub/WWW/TheProject.html HTTP/1.1`
  AbsoluteUri(&'a str),

  // The most common form of Request-URI is that used to identify a resource on
  // an origin server or gateway. For example, a client wishing to retrieve a
  // resource directly from the origin server would create a TCP connection to
  // port 80 of the host "www.w3.org" and send the following lines:
  // ```
  // GET /pub/WWW/TheProject.html HTTP/1.1
  // Host: www.w3.org
  // ```
  // Note that the absolute path cannot be empty; if none is present in the
  // original URI, it MUST be given as "/" (the server root).
  AbsolutePath(&'a str),
}

/// Enum of Header field
#[derive(Debug)]
enum HTTPRequestHeader<'a> {
  AcceptEncoding(&'a str),
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

impl Into<HttpMethod> for &str {
  fn into(self) -> HttpMethod {
    match self.to_ascii_uppercase().as_str() {
      "GET" => HttpMethod::Get,
      "HEAD" => HttpMethod::Head,
      "POST" => HttpMethod::Post,
      "PUT" => HttpMethod::Put,
      "DELETE" => HttpMethod::Delete,
      "CONNECT" => HttpMethod::Connect,
      "OPTION" => HttpMethod::Option,
      "TRACE" => HttpMethod::Trace,
      _ => panic!("Bad HTTP Method [{}]!", self)
    }
  }
}

impl<'a> Into<RequestURI<'a>> for &'a str {
  fn into(self) -> RequestURI<'a> {
    match self {
      "*" => RequestURI::Asterisk,
      _ => {
        if self.contains("://") {
          RequestURI::AbsoluteUri(self)
        } else if self.starts_with('/') {
          RequestURI::AbsolutePath(self)
        } else {
          panic!("Bad Request URI [{}]!", self)
        }
      }
    }
  }
}

impl<'a> Into<HTTPRequestHeader<'a>> for &'a str {
  fn into(self) -> HTTPRequestHeader<'a> {
    match self.to_ascii_uppercase().as_str() {
      "ACCEPT-ENCODING" => HTTPRequestHeader::AcceptEncoding(&self[17..]),
      "HOST" => HTTPRequestHeader::Host(&self[6..]),
      "REFERER" => HTTPRequestHeader::Referer(&self[9..]),
      "USER-AGENT" => HTTPRequestHeader::UserAgent(&self[12..]),
      _ => {
        let colon = self.find(':').expect(
          format!("Failed to find a colon in [{}]!", self).as_str()
        );
        HTTPRequestHeader::_OtherHeader(&self[..colon],
                                        &self[(colon + 2)..])
      }
    }
  }
}