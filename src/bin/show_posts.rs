extern crate rustfolio;
extern crate diesel;

use self::rustfolio::*;
use self::rustfolio::models::*;
use self::diesel::prelude::*;

fn main() {
    use rustfolio::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
      .limit(5)
      .load::<Post>(&connection)
      .expect("Error loading posts");

    println!("Displaying {} posts", results.len());

    for post in results {
      println!("{}", post.title);
      println!("---------\n");
      println!("{}", post.body);
    }

}
