extern crate rocket_contrib;
extern crate diesel;

use self::rocket_contrib::Json;
use self::diesel::prelude::*;

#[get("/")]
fn all_posts() -> Json<&'static str> {
    Json("{ allPosts: true }")
}

#[get("/<_id>")]
fn post(_id: usize) -> Json<&'static str> {
    Json("{hi: 'ojla'}")
}
