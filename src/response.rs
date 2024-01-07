use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[allow(dead_code)]
pub enum StatusCode {
    Continue,
    SwitchingProtocols,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    RequestUriTooLarge,
    UnsupportedMediaType,
    RequestedRangeNotSatisfiable,
    ExpectationFailed,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported
}

impl StatusCode {
    /// internal convenience func used in display trait implementation
    fn get_text(&self) -> &str {
        use StatusCode::*;
        match self {
            Continue => "100 Continue",
            SwitchingProtocols => "101 Switching Protocols",
            Ok => "200 OK",
            Created => "201 Created",
            Accepted => "202 Accepted",
            NonAuthoritativeInformation => "203 Non-Authoritative Information",
            NoContent => "204 No Content",
            ResetContent => "205 Reset Content",
            PartialContent => "206 Partial Content",
            MultipleChoices => "300 Multiple Choices",
            MovedPermanently => "301 Moved Permanently",
            Found => "302 Found",
            SeeOther => "303 See Other",
            NotModified => "304 Not Modified",
            UseProxy => "305 Use Proxy",
            TemporaryRedirect => "307 Temporary Redirect",
            BadRequest => "400 Bad Request",
            Unauthorized => "401 Unauthorized",
            PaymentRequired => "402 Payment Required",
            Forbidden => "403 Forbidden",
            NotFound => "404 Not Found",
            MethodNotAllowed => "405 Method Not Allowed",
            NotAcceptable => "406 Not Acceptable",
            ProxyAuthenticationRequired => "407 Proxy Authentication Required",
            RequestTimeout => "408 Request Time-out",
            Conflict =>  "409 Conflict",
            Gone => "410 Gone",
            LengthRequired => "411 Length Required",
            PreconditionFailed => "412 Precondition Failed",
            RequestEntityTooLarge => "413 Request Entity Too Large",
            RequestUriTooLarge => "414 Request-URI Too Large",
            UnsupportedMediaType => "415 Unsupported Media Type",
            RequestedRangeNotSatisfiable => "416 Requested range not satisfiable",
            ExpectationFailed => "417 Expectation Failed",
            InternalServerError => "500 Internal Server Error",
            NotImplemented => "501 Not Implemented",
            BadGateway => "502 Bad Gateway",
            ServiceUnavailable => "503 Service Unavailable",
            GatewayTimeout => "504 Gateway Time-out",
            HttpVersionNotSupported => "505 HTTP Version not supported"
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_text())
    }
}

#[derive(Clone, Debug)]
pub struct Response {
    pub status: StatusCode,
    pub body: String,
    pub headers: Vec<String>
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP/1.1 {status}\r\nContent-Length: {length}\r\n",
            status = self.status.to_string(),
            length = self.body.len(),
        )?;

        for el in &self.headers {
            write!(f, "{el}\r\n")?;
        }

        write!(f, "\r\n{}", self.body)
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: StatusCode::Ok,
            body: String::new(),
            headers: Vec::new()
        }
    }
}
