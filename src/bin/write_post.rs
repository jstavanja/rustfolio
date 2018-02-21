extern crate rustfolio;
extern crate diesel;

use self::rustfolio::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Enter title:");
    let mut title = String::new();

    stdin().read_line(&mut title).unwrap();

    let title = &title[..(title.len() - 1)]; // Drop newline

    println!("\nOk! Let's write {} (Press {} when finished)\n",
            title,
            EOF);

    let mut body = String::new();

    stdin().read_to_string(&mut body).unwrap();

    let _ = create_post(&connection, title, &body);
    println!("\nSaved draft {}", title);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
