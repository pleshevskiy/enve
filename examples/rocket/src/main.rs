#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use itconfig::config;

config! {
    ROCKET {
        HOST: String => "localhost",
        PORT: u16 => 9000,
        BASE_URL => "/",
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    config::init();

    rocket::ignite()
        .mount(config::ROCKET::BASE_URL(), routes![index])
        .launch();
}