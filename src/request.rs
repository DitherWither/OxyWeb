//! HTTP Requests and related code

use std::{
    io::{self, BufRead, BufReader, Read},
    net::TcpStream,
    str,
};

/// HTTP Method, as defined under RFC 2616
///
/// Is `Unknown` when the server was unable to parse the method
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Method {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
    Unknown,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        use self::Method::*;
        match value {
            "OPTIONS" => Options,
            "GET" => Get,
            "HEAD" => Head,
            "POST" => Post,
            "PUT" => Put,
            "DELETE" => Delete,
            "TRACE" => Trace,
            "CONNECT" => Connect,
            _ => Unknown,
        }
    }
}

/// A HTTP request
#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl Request {
    /// Parse a http request from a tcp stream
    ///
    /// TODO: Document the internals of this function
    pub fn parse(mut reader: BufReader<&mut TcpStream>) -> io::Result<Self> {
        let mut status_line = String::new();
        reader.read_line(&mut status_line)?;
        let status_line = status_line.split(' ').collect::<Vec<&str>>();

        let mut headers = Vec::new();
        let mut is_body = false;
        let mut length = 0;
        let mut body = String::new();

        loop {
            if is_body {
                if length > 4096 {
                    break;
                }
                let mut body_bytes: Vec<u8> = vec![0; length];
                reader.read_exact(body_bytes.as_mut_slice())?;
                body = str::from_utf8(&body_bytes)
                    .or(Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Couldn't convert body to utf8",
                    )))?
                    .to_string();
                break;
            } else {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => (),
                    Err(_) => break,
                }
                if line == "\r\n" && !is_body {
                    if length == 0 {
                        break;
                    }
                    is_body = true;
                    continue;
                }
                line = line.replace("\r\n", "");
                headers.push(line.clone());
                let line_split: Vec<_> = line.split(':').collect();

                if line_split[0] == "Content-Length" {
                    let l = line_split[1].trim().parse::<usize>();
                    if let Ok(l) = l {
                        length = l;
                    }
                }
            }
        }

        Ok(Request {
            method: status_line[0].into(),
            path: status_line[1].into(),
            version: status_line[2].into(),
            headers,
            body,
        })
    }
}
