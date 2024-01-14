//! A example hello world application

use oxyweb::*;

pub struct Application;

impl HttpApplication for Application {
    fn handle_request(&self, _req: Request) -> Option<Response> {
        None
    }
}

fn main() {
    oxyweb::http::run(Application);
}