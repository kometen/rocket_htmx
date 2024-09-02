// https://kojinglick.com/using-htmx-with-rust-quickstart
// https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-3-3f9b7d511bde

mod index;
mod password_counter;
mod password_generator;
mod password_length;
mod password_options;

#[macro_use]
extern crate rocket;

use crate::index::{root, static_files};
use crate::password_counter::{decrement_password_count, increment_password_count};
use crate::password_generator::generate_passwords;
use crate::password_length::{decrement_password_length, increment_password_length};
use crate::password_options::*;
use std::sync::atomic::{AtomicBool, AtomicU8};

pub struct PasswordAttributes {
    count: AtomicU8,
    length: AtomicU8,
    numbers: AtomicBool,
    lowercase_letters: AtomicBool,
    uppercase_letters: AtomicBool,
    symbols: AtomicBool,
    spaces: AtomicBool,
    exclude_similar_characters: AtomicBool,
    strict: AtomicBool,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(PasswordAttributes {
            count: AtomicU8::new(5),
            length: AtomicU8::new(6),
            numbers: AtomicBool::new(true),
            lowercase_letters: AtomicBool::new(true),
            uppercase_letters: AtomicBool::new(true),
            symbols: AtomicBool::new(true),
            spaces: AtomicBool::new(false),
            exclude_similar_characters: AtomicBool::new(false),
            strict: AtomicBool::new(false),
        })
        .mount(
            "/",
            routes![
                root,
                numbers_option,
                change_numbers_option,
                lowercase_letters_option,
                change_lowercase_letters_option,
                uppercase_letters_option,
                change_uppercase_letters_option,
                symbols_option,
                change_symbols_option,
                spaces_option,
                change_spaces_option,
                exclude_similar_characters_option,
                change_exclude_similar_characters_option,
                decrement_password_count,
                decrement_password_length,
                generate_passwords,
                increment_password_count,
                increment_password_length,
                static_files,
            ],
        )
}
