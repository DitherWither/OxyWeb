use std::fs;

use crate::prelude::{Response, StatusCode};

pub fn serve_file(path: &str, status: StatusCode) -> Response {
    Response {
        body: fs::read_to_string(path).unwrap(),
        status,
        ..Default::default()
    }
}