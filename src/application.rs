use crate::prelude::*;

pub struct Application;

impl HttpApplication for Application {
    fn handle_request(&self, req: Request) -> Response {
        use Method::*;
        match (req.method, req.path.as_str()) {
            (Get, "/") => serve_file("index.html", StatusCode::Ok),
            (_, _) => serve_file("404.html", StatusCode::NotFound),
        }
    }
}
