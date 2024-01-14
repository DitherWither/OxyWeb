//! Contains core HTTP logic that handles requests and responses
use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    sync::Arc,
};

use crate::{
    config::Config, thread_pool::ThreadPool, utility::serve_file, Method, Request, Response,
    StatusCode,
};

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
        for stream in self.listener.incoming().flatten() {
            let app_clone = Arc::clone(&self.application);
            self.pool.execute(move || {
                HttpServer::handle_connection(&app_clone, stream);
            });
        }
    }

    /// Handle a single http request
    fn handle_connection(app: &Arc<T>, mut stream: TcpStream) {
        if let Ok(req) = Request::parse(BufReader::new(&mut stream)) {
            let response = match app.handle_request(req.clone()) {
                Some(response) => response,
                None => serve_static_file(req),
            };
            stream.write_all(response.to_string().as_bytes()).unwrap();
        } else {
            let response = serve_file("res/bad_request.html", StatusCode::BadRequest);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }
}

fn serve_static_file(request: Request) -> Response {
    if request.method != Method::Get {
        return serve_file("res/404.html", StatusCode::NotFound);
    }
    let mut path = format!(
        "res/{}",
        request.path.strip_prefix('/').unwrap_or(&request.path)
    );

    if path.ends_with('/') {
        path += "index.html";
    }

    path = path.replace("../", "");

    if PathBuf::from(path.clone()).exists() {
        serve_file(&path, StatusCode::Ok)
    } else {
        serve_file("res/404.html", StatusCode::NotFound)
    }
}
/// A trait for applications using the http server
pub trait HttpApplication: Send + Sync {
    /// Request handler, called for every request
    ///
    /// If the return value is None, a static file will be served instead
    fn handle_request(&self, req: Request) -> Option<Response>;
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
