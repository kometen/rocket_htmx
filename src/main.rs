// https://kojinglick.com/using-htmx-with-rust-quickstart
// https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-3-3f9b7d511bde

mod password_counter;
mod password_length;
mod index;

#[macro_use] extern crate rocket;

use std::sync::atomic::{AtomicU8};
use crate::index::{root, static_files};
use crate::password_counter::{decrement_password_count, increment_password_count};
use crate::password_length::{decrement_password_length, increment_password_length};

struct PasswordAttributes {
    count: AtomicU8,
    length: AtomicU8,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(PasswordAttributes { count: AtomicU8::new(5), length: AtomicU8::new(6)})
        .mount("/", routes![
            root,
            decrement_password_count,
            increment_password_count,
            decrement_password_length,
            increment_password_length,
            static_files,
        ])
}
