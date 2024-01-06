mod config;
mod request;
mod response;

use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream}, fs,
};

use request::Request;

use crate::{config::Config, response::Response, request::Method};

fn main() {
    let config = Config::load();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_connection(stream);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let req = Request::parse(buf_reader);
    if let Ok(req) = req {
        let response = handle_request(req);
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}

fn handle_request(req: Request) -> Response {
    if req.path == "/" && req.method == Method::Get {
        return Response {
            body: fs::read_to_string("index.html").unwrap(),
            ..Default::default()
        }
    }

    Response {
        body: fs::read_to_string("404.html").unwrap(),
        status: response::StatusCode::NotFound,
        ..Default::default()
    }
}