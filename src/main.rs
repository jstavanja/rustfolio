#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod dashboard;

use rocket::response::content;

#[get("/")]
fn index() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

#[get("/test")]
fn test() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'tester' }")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, test])
        .mount("/dashboard", routes![dashboard::settings])
        .launch();
}
