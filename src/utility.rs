//! Commonly used utility functions
use std::fs;

use crate::{Response, StatusCode};

/// Serve a static file with specified status code
pub fn serve_file(path: &str, status: StatusCode) -> Response {
    Response {
        body: fs::read_to_string(path).unwrap(),
        status,
        ..Default::default()
    }
}
