mod config;
mod request;
mod response;
mod thread_pool;
mod http;
mod application;
mod prelude;
mod utility;

use crate::{http::run, application::Application};

fn main() {
    run(Application);
}