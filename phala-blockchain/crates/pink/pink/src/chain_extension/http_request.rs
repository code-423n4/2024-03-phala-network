use alloc::string::String;
use alloc::vec::Vec;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use super::ErrorCode;
#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    /// Create a new http request.
    pub fn new(
        url: impl Into<String>,
        method: impl Into<String>,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
    ) -> Self {
        Self {
            url: url.into(),
            method: method.into(),
            headers,
            body,
        }
    }
}

#[derive(scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct HttpResponse {
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(
    scale::Encode, scale::Decode, TryFromPrimitive, IntoPrimitive, Clone, Copy, Debug, PartialEq, Eq,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[repr(u32)]
pub enum HttpRequestError {
    InvalidUrl,
    InvalidMethod,
    InvalidHeaderName,
    InvalidHeaderValue,
    FailedToCreateClient,
    Timeout,
    NotAllowed,
    TooManyRequests,
    NetworkError,
    ResponseTooLarge,
}

impl super::sealed::Sealed for HttpRequestError {}
impl super::CodableError for HttpRequestError {
    fn encode(&self) -> u32 {
        let code: u32 = (*self).into();
        code + 1
    }

    fn decode(code: u32) -> Option<Self> {
        if code == 0 {
            return None;
        }
        core::convert::TryFrom::try_from(code - 1).ok()
    }
}

impl From<ErrorCode> for HttpRequestError {
    fn from(value: ErrorCode) -> Self {
        match <Self as super::CodableError>::decode(value.0) {
            None => crate::panic!("chain extension: invalid HttpRequestError"),
            Some(err) => err,
        }
    }
}

impl From<scale::Error> for HttpRequestError {
    fn from(_value: scale::Error) -> Self {
        crate::panic!("chain_ext: failed to decocde HttpRequestError")
    }
}

impl HttpRequestError {
    pub fn display(&self) -> &'static str {
        match self {
            Self::InvalidUrl => "Invalid URL",
            Self::InvalidMethod => "Invalid method",
            Self::InvalidHeaderName => "Invalid header name",
            Self::InvalidHeaderValue => "Invalid header value",
            Self::FailedToCreateClient => "Failed to create client",
            Self::Timeout => "Timeout",
            Self::NotAllowed => "Not allowed",
            Self::TooManyRequests => "Too many requests",
            Self::NetworkError => "Network error",
            Self::ResponseTooLarge => "Response too large",
        }
    }
}

impl HttpResponse {
    pub fn ok(body: Vec<u8>) -> Self {
        Self {
            status_code: 200,
            reason_phrase: "OK".into(),
            headers: Default::default(),
            body,
        }
    }

    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            reason_phrase: "Not Found".into(),
            headers: Default::default(),
            body: Default::default(),
        }
    }
}

#[macro_export]
macro_rules! http_req {
    ($method: expr, $url: expr, $data: expr, $headers: expr) => {{
        use $crate::chain_extension::HttpRequest;
        let headers = $headers;
        let body = $data;
        let request = HttpRequest {
            url: $url.into(),
            method: $method.into(),
            headers,
            body,
        };
        $crate::ext().http_request(request)
    }};
    ($method: expr, $url: expr, $data: expr) => {{
        $crate::http_req!($method, $url, $data, Default::default())
    }};
}

/// Make a simple HTTP GET request
///
/// # Arguments
/// - url: The URL to GET
/// - headers: The headers to send with the request
///
/// # Examples
///
/// ```ignore
/// use pink::http_get;
/// let response = http_get!("https://example.com/");
/// assert_eq!(response.status_code, 200);
/// ```
///
/// ```ignore
/// use pink::http_get;
/// let headers = vec![("X-Foo".into(), "Bar".into())];
/// let response = http_get!("https://example.com/", headers);
/// assert_eq!(response.status_code, 200);
/// ```
#[macro_export]
macro_rules! http_get {
    ($url: expr, $headers: expr) => {{
        $crate::http_req!("GET", $url, Default::default(), $headers)
    }};
    ($url: expr) => {{
        $crate::http_get!($url, Default::default())
    }};
}

/// Make a simple HTTP POST request
///
/// # Arguments
/// - url: The URL to POST
/// - data: The payload to POST
/// - headers: The headers to send with the request
///
/// # Examples
///
/// ```ignore
/// use pink::http_post;
/// let response = http_post!("https://example.com/", b"Hello, world!");
/// assert_eq!(response.status_code, 200);
/// ```
///
/// ```ignore
/// use pink::http_post;
/// let headers = vec![("X-Foo".into(), "Bar".into())];
/// let response = http_post!("https://example.com/", b"Hello, world!", headers);
/// assert_eq!(response.status_code, 200);
/// ```
#[macro_export]
macro_rules! http_post {
    ($url: expr, $data: expr, $headers: expr) => {{
        $crate::http_req!("POST", $url, $data.into(), $headers)
    }};
    ($url: expr, $data: expr) => {{
        $crate::http_post!($url, $data, Default::default())
    }};
}

/// Make a simple HTTP PUT request
///
/// # Arguments
/// - url: The destination URL
/// - data: The payload to PUT
/// - headers: The headers to send with the request
///
/// # Examples
///
/// ```ignore
/// use pink::http_put;
/// let response = http_put!("https://example.com/", b"Hello, world!");
/// assert_eq!(response.status_code, 200);
/// ```
#[macro_export]
macro_rules! http_put {
    ($url: expr, $data: expr, $headers: expr) => {{
        $crate::http_req!("PUT", $url, $data.into(), $headers)
    }};
    ($url: expr, $data: expr) => {{
        $crate::http_put!($url, $data, Default::default())
    }};
}
