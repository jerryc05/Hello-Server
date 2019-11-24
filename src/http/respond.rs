//! Sample HTTP Respond Format
//! ```
//! A Status-line (HTTP-Version SP Status-Code SP Reason-Phrase CRLF)
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
//! HTTP/1.1 200 OK
//! Date: Sun, 18 Oct 2012 10:36:20 GMT
//! Server: Apache/2.2.14 (Win32)
//! Content-Length: 230
//! Content-Type: text/html; charset=iso-8859-1
//! Connection: Closed
//!
//! {status:200,msg:"OK"}
//! ```

use crate::http::version::HttpVersion;

/// Struct of parsed HTTP Respond
#[derive(Debug)]
pub struct HTTPRespond<'a> {
  // First line
  pub http_version: HttpVersion,
  pub status_code: StatusCode,
  pub reason_phrase: &'a str,

  // Header fields
  pub header: Vec<HttpRespondHeader<'a>>,

  // Body field
  pub body: &'a str,
}

#[allow(dead_code)]
impl<'a> HTTPRespond<'a> {
  fn from_body(body: &'a str,
               http_version: HttpVersion,
               status_code: StatusCode,
               reason_phrase: &'a str) -> HTTPRespond<'a> {
    HTTPRespond {
      http_version,
      status_code,
      reason_phrase,
      header: Vec::new(),
      body,
    }
  }

  fn with_header(respond: &mut HTTPRespond<'a>,
                 header: HttpRespondHeader<'a>) {
    respond.header.push(header);
  }
}

/// Enum of HTTP Status Code field
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
pub enum StatusCode {
  Continue = 100,
  SwitchingProtocols = 101,
  Processing = 102,

  Ok = 200,
  Created = 201,
  Accepted = 202,
  NonAuthoritativeInformation = 203,
  NoContent = 204,
  ResetContent = 205,
  PartialContent = 206,
  MultiStatus = 207,
  AlreadyReported = 208,
  IMUsed = 226,

  MultipleChoices = 300,
  MovedPermanently = 301,
  Found = 302,
  SeeOther = 303,
  NotModified = 304,
  UseProxy = 305,
  TemporaryRedirect = 307,
  PermanentRedirect = 308,

  BadRequest = 400,
  Unauthorized = 401,
  PaymentRequired = 402,
  Forbidden = 403,
  NotFound = 404,
  MethodNotAllowed = 405,
  NotAcceptable = 406,
  ProxyAuthenticationRequired = 407,
  RequestTimeout = 408,
  Conflict = 409,
  Gone = 410,
  LengthRequired = 411,
  PreconditionFailed = 412,
  PayloadTooLarge = 413,
  URITooLong = 414,
  UnsupportedMediaType = 415,
  RangeNotSatisfiable = 416,
  ExpectationFailed = 417,
  ImATeapot = 418,
  MisdirectedRequest = 421,
  UnprocessableEntity = 422,
  Locked = 423,
  FailedDependency = 424,
  UpgradeRequired = 426,
  PreconditionRequired = 428,
  TooManyRequests = 429,
  RequestHeaderFieldsTooLarge = 431,
  UnavailableForLegalReasons = 451,

  InternalServerError = 500,
  NotImplemented = 501,
  BadGateway = 502,
  ServiceUnavailable = 503,
  GatewayTimeout = 504,
  HTTPVersionNotSupported = 505,
  VariantAlsoNegotiates = 506,
  InsufficientStorage = 507,
  LoopDetected = 508,
  NotExtended = 510,
  NetworkAuthenticationRequired = 511,
}

/// Enum of Header field
#[allow(dead_code)]
#[derive(Debug)]
pub enum HttpRespondHeader<'a> {
  Age(&'a str),
  ContentEncoding(&'a str),
  ContentLength(&'a str),
  ContentType(&'a str),
  Server(&'a str),
  _OtherHeader(&'a str, &'a str),
}