use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum StatusCode {
    Ok,
    NotFound,
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusCode::Ok => write!(f, "200 OK"),
            StatusCode::NotFound => write!(f, "404 Not Found")
        }
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
