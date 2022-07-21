#[macro_use]
extern crate rocket;

itconfig::config! {
    rocket {
        HOST: String => "localhost",
        PORT: u16 => 9000,
        BASE_URL => "/",
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    config::init();

    rocket::build().mount(config::rocket::BASE_URL(), routes![hello])
}
