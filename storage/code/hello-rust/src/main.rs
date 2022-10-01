extern crate core;

use crate::domain::user::vo::id::Id;
use crate::domain::user::vo::name::Name;

mod domain;
mod application;
mod configuration;
mod infrastructure;

fn main() {
    let conf = configuration::server::Server::new();
    conf.use_app();
}
