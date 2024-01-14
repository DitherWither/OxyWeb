//! A example hello world application

use oxyweb::*;

pub struct Application;

impl HttpApplication for Application {
    fn handle_request(&self, req: Request) -> Option<Response> {
        if req.path == "/hello" && req.method == Method::Get {
            Some(Response { status: StatusCode::Ok, body: "Hello, World".to_owned(), ..Default::default() })
        } else {
            None
        }
    }
}

fn main() {
    oxyweb::http::run(Application);
}
