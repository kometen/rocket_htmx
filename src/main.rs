// https://kojinglick.com/using-htmx-with-rust-quickstart
// https://medium.com/perimeterx/late-night-confessions-building-a-website-using-rust-rocket-diesel-and-askama-part-3-3f9b7d511bde

#[macro_use] extern crate rocket;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use rocket::response::status::NotFound;
use askama_rocket::Template;
use rocket::fs::NamedFile;
use rocket::State;

struct PasswordAttributes {
    count: AtomicU8,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
    password_count_value: u8,
}

#[get("/")]
async fn root(password_count: &State<PasswordAttributes>) -> Result<IndexTemplate, NotFound<String>> {
    let template = IndexTemplate {
        name: "World".to_string(),
        password_count_value: password_count.count.load(Ordering::Relaxed),
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
        .manage(PasswordAttributes { count: AtomicU8::new(5)})
        .mount("/", routes![root, static_files])
}
