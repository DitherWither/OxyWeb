//! A example hello world application

use oxyweb::*;

pub struct Application;

impl HttpApplication for Application {
    fn handle_request(&self, req: Request) -> Response {
        use Method::*;
        match (req.method, req.path.as_str()) {
            (Get, "/") => utility::serve_file("res/index.html", StatusCode::Ok),
            (_, _) => utility::serve_file("res/404.html", StatusCode::NotFound),
        }
    }
}

fn main() {
    oxyweb::http::run(Application);
}