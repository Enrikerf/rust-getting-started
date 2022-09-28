use crate::domain::user::vo::id::Id;
use crate::domain::user::vo::name::Name;

mod domain;
mod application;
mod configuration;
mod infrastructure;

fn main() {
    let conf = configuration::server::Server::new();
    let user = domain::user::User::new(
        Id { value: Default::default() },
        Name { value: "name".to_string() }
    );
    conf.use_app();
    println!("{}",user.get_events()[0].get_id().value);
}
