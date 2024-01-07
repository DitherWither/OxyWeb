use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{prelude::*, thread_pool::ThreadPool, config::Config};

pub struct HttpServer<T>
where
    T: HttpApplication,
{
    listener: TcpListener,
    pool: ThreadPool,
    application: T,
}

impl<T> HttpServer<T>
where
    T: HttpApplication,
{
    pub fn new(config: &Config, application: T) -> Self {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();
        let pool = ThreadPool::new(8);

        Self {
            listener,
            pool,
            application,
        }
    }

    pub fn run(&'static self) {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                self.pool.execute(|| {
                    self.handle_connection(stream);
                });
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let req = Request::parse(buf_reader);
        let response = if let Ok(req) = req {
            self.application.handle_request(req)
        } else {
            serve_file("res/bad_request.html", StatusCode::BadRequest)
        };
        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}

pub trait HttpApplication: Send + Sync {
    fn handle_request(&self, req: Request) -> Response;

    fn run(&self)
    where
        Self: HttpApplication,
    {
    }
}

pub fn run<T>(application: T)
where
    T: HttpApplication + 'static,
{
    let config = Config::load();
    let server = HttpServer::new(&config, application);
    let boxed = Box::new(server);
    let b = Box::leak(boxed);

    b.run();
}
