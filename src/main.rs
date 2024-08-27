// https://kojinglick.com/using-htmx-with-rust-quickstart
// https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-3-3f9b7d511bde

mod password_counter;
mod password_length;

#[macro_use] extern crate rocket;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use rocket::response::status::NotFound;
use askama_rocket::{Template};
use rocket::fs::NamedFile;
use rocket::State;
use crate::password_counter::{decrement_password_count, increment_password_count};
use crate::password_length::{decrement_password_length, increment_password_length};

struct PasswordAttributes {
    count: AtomicU8,
    length: AtomicU8,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
    password_count_value: u8,
    password_length_value: u8,
}

#[get("/")]
async fn root(password_count: &State<PasswordAttributes>) -> Result<IndexTemplate, NotFound<String>> {

    println!("Initial password count: {}", password_count.count.load(Ordering::Relaxed));

    let template = IndexTemplate {
        name: "World".to_string(),
        password_count_value: password_count.count.load(Ordering::Relaxed),
        password_length_value: password_count.length.load(Ordering::Relaxed),
    };

    let response = template;
    Ok(response)
}

#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("site").join(path);
    NamedFile::open(path)
        .await
        .map_err(|e| NotFound(e.to_string()))
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
