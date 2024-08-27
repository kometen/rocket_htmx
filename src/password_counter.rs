use std::sync::atomic::Ordering;
use askama::Template;
use rocket::response::status::NotFound;
use rocket::State;
use crate::PasswordAttributes;

#[derive(Template)]
#[template(path = "components/password_counter.html")]
pub struct PasswordCounterTemplate {
    password_count_value: u8,
}

#[get("/increment_password_count")]
pub async fn increment_password_count(password_attribute: &State<PasswordAttributes>) -> Result<PasswordCounterTemplate, NotFound<String>> {

    let c = password_attribute.count.load(Ordering::Relaxed) + 1;

    if c < 31 {
        password_attribute.count.store(c, Ordering::Relaxed);
    }

    let template = PasswordCounterTemplate {
        password_count_value: password_attribute.count.load(Ordering::Relaxed),
    };
    println!("Increment password count: {}", password_attribute.count.load(Ordering::Relaxed));

    let response = template;
    Ok(response)
}

#[get("/decrement_password_count")]
pub async fn decrement_password_count(password_attribute: &State<PasswordAttributes>) -> Result<PasswordCounterTemplate, NotFound<String>> {

    let c = password_attribute.count.load(Ordering::Relaxed) - 1;

    if c > 0 {
        password_attribute.count.store(c, Ordering::Relaxed);
    }

    let template = PasswordCounterTemplate {
        password_count_value: password_attribute.count.load(Ordering::Relaxed),
    };
    println!("Decrement password count: {}", password_attribute.count.load(Ordering::Relaxed));

    let response = template;
    Ok(response)
}
