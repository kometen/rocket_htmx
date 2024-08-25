#[macro_use] extern crate rocket;

use std::path::PathBuf;
use rocket::response::status::NotFound;
use askama_rocket::Template;
use rocket::fs::NamedFile;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

#[get("/")]
async fn root() -> Result<IndexTemplate, NotFound<String>> {
    let template = IndexTemplate {
        name: "World".to_string(),
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
    rocket::build().mount("/", routes![root, static_files])
}
