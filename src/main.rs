mod config;
mod request;
mod response;
mod threadpool;

use std::{
    fs,
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use request::Request;
use threadpool::ThreadPool;

use crate::{config::Config, request::Method, response::Response};

fn main() {
    let config = Config::load();

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            pool.execute(|| {
                handle_connection(stream);
            });
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
    use Method::*;
    match (req.method, req.path.as_str()) {
        (Get, "/") => Response {
            body: fs::read_to_string("index.html").unwrap(),
            ..Default::default()
        },
        (Get, "/sleep") => {
            thread::sleep(Duration::from_secs(5));
            Response {
                body: fs::read_to_string("index.html").unwrap(),
                ..Default::default()
            }
        }
        (_, _) => Response {
            body: fs::read_to_string("404.html").unwrap(),
            status: response::StatusCode::NotFound,
            ..Default::default()
        },
    }
}
