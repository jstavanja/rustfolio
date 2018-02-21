#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod dashboard;
mod blog;

use rocket::response::content;

#[get("/")]
fn index() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/dashboard", routes![dashboard::settings])
        .mount("/posts", routes![blog::post, blog::all_posts])
        .launch();
}
