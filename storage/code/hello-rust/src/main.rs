mod domain;
mod application;
mod configuration;
mod infrastructure;

fn main() {
    let conf = configuration::configuration::Configuration::new();
    conf.use_app();
    println!("Hello, world!");
}
