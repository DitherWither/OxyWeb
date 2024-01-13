mod application;
mod config;
mod http;
mod prelude;
mod request;
mod response;
mod thread_pool;
mod utility;

use crate::{application::Application, http::run};

fn main() {
    run(Application);
}
