//! Contains core HTTP logic that handles requests and responses
use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{config::Config, Request, Response, StatusCode, utility::serve_file, thread_pool::ThreadPool};

/// A Multi-Threaded http server
pub struct HttpServer<T>
where
    T: HttpApplication + 'static,
{
    listener: TcpListener,
    pool: ThreadPool,
    application: Arc<T>,
}

impl<T> HttpServer<T>
where
    T: HttpApplication + 'static,
{
    /// Create a new instance of the HTTP server, and bind the port provided by the config
    pub fn new(config: &Config, application: T) -> Self {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();
        let pool = ThreadPool::new(8);

        Self {
            listener,
            pool,
            application: Arc::new(application),
        }
    }

    /// Run the event loop for the server
    pub fn run(self) {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                let app_clone = Arc::clone(&self.application);
                self.pool.execute(move || {
                    HttpServer::handle_connection(&app_clone, stream);
                });
            }
        }
    }

    /// Handle a single http request
    fn handle_connection(app: &Arc<T>, mut stream: TcpStream) {
        if let Ok(req) = Request::parse(BufReader::new(&mut stream)) {
            let response = app.handle_request(req);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        } else {
            let response = serve_file("res/bad_request.html", StatusCode::BadRequest);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }
}

/// A trait for applications using the http server
pub trait HttpApplication: Send + Sync {
    /// Request handler, called for every request
    fn handle_request(&self, req: Request) -> Response;
}

/// Start an application up, creating the server and loading the config
pub fn run<T>(application: T)
where
    T: HttpApplication + 'static,
{
    let config = Config::load();
    let server = HttpServer::new(&config, application);
    server.run();
}
