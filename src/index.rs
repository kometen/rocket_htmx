use crate::PasswordAttributes;
use askama::Template;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::State;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use crate::password_generator::Pwd;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    name: String,
    password_count_value: u8,
    password_length_value: u8,
    foo: String,
    passwords: Vec<Pwd>,
}

#[get("/")]
pub async fn root(
    password_count: &State<PasswordAttributes>,
) -> Result<IndexTemplate, NotFound<String>> {
    println!(
        "Initial password count: {}",
        password_count.count.load(Ordering::Relaxed)
    );

    let template = IndexTemplate {
        name: "World".to_string(),
        password_count_value: password_count.count.load(Ordering::Relaxed),
        password_length_value: password_count.length.load(Ordering::Relaxed),
        foo: "bar".to_string(),
        passwords: vec![],
    };

    let response = template;
    Ok(response)
}

#[get("/<path..>")]
pub async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("site").join(path);
    NamedFile::open(path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}
