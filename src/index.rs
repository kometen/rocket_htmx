use crate::password_generator::Pwd;
use crate::PasswordAttributes;
use askama::Template;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::State;
use std::path::PathBuf;
use std::sync::atomic::Ordering;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    name: String,
    password_count_value: u8,
    password_length_value: u8,
    passwords: Vec<Pwd>,
    numbers_checkbox: String,
    lowercase_letters_checkbox: String,
    uppercase_letters_checkbox: String,
    symbols_checkbox: String,
    spaces_checkbox: String,
    exclude_similar_characters_checkbox: String,
}

#[get("/")]
pub async fn root(
    password_attributes: &State<PasswordAttributes>,
) -> Result<IndexTemplate, NotFound<String>> {
    let number_checkbox_status = match password_attributes.numbers.load(Ordering::Relaxed) {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let lowercase_letters_checkbox_status = match password_attributes
        .lowercase_letters
        .load(Ordering::Relaxed)
    {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let uppercase_letters_checkbox_status = match password_attributes
        .uppercase_letters
        .load(Ordering::Relaxed)
    {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let symbols_checkbox_status = match password_attributes.symbols.load(Ordering::Relaxed) {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let spaces_checkbox_status = match password_attributes.spaces.load(Ordering::Relaxed) {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let exclude_similar_characters_checkbox_status = match password_attributes
        .exclude_similar_characters
        .load(Ordering::Relaxed)
    {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let template = IndexTemplate {
        name: "World".to_string(),
        password_count_value: password_attributes.count.load(Ordering::Relaxed),
        password_length_value: password_attributes.length.load(Ordering::Relaxed),
        passwords: vec![],
        numbers_checkbox: number_checkbox_status,
        lowercase_letters_checkbox: lowercase_letters_checkbox_status,
        uppercase_letters_checkbox: uppercase_letters_checkbox_status,
        symbols_checkbox: symbols_checkbox_status,
        spaces_checkbox: spaces_checkbox_status,
        exclude_similar_characters_checkbox: exclude_similar_characters_checkbox_status,
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
