use crate::PasswordAttributes;
use askama::Template;
use rocket::response::status::NotFound;
use rocket::State;
use std::sync::atomic::Ordering;

#[derive(Template)]
#[template(path = "components/password_length.html")]
pub struct PasswordLengthTemplate {
    password_length_value: u8,
}

#[get("/increment_password_length")]
pub async fn increment_password_length(
    password_attribute: &State<PasswordAttributes>,
) -> Result<PasswordLengthTemplate, NotFound<String>> {
    let c = password_attribute.length.load(Ordering::Relaxed) + 1;

    if c < 26 {
        password_attribute.length.store(c, Ordering::Relaxed);
    }

    let template = PasswordLengthTemplate {
        password_length_value: password_attribute.length.load(Ordering::Relaxed),
    };

    let response = template;
    Ok(response)
}

#[get("/decrement_password_length")]
pub async fn decrement_password_length(
    password_attribute: &State<PasswordAttributes>,
) -> Result<PasswordLengthTemplate, NotFound<String>> {
    let c = password_attribute.length.load(Ordering::Relaxed) - 1;

    if c > 5 {
        password_attribute.length.store(c, Ordering::Relaxed);
    }

    let template = PasswordLengthTemplate {
        password_length_value: password_attribute.length.load(Ordering::Relaxed),
    };

    let response = template;
    Ok(response)
}
