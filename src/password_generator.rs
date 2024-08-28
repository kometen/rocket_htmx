use crate::PasswordAttributes;
use askama::Template;
use passwords::{analyzer, scorer, PasswordGenerator};
use rocket::response::status::NotFound;
use rocket::State;
use std::sync::atomic::Ordering;

#[derive(serde::Serialize, Debug)]
pub struct Pwd {
    pub password: String,
    pub score: u8,
}

#[derive(Template, Debug)]
#[template(path = "components/passwords.html")]
pub struct PasswordsTemplate {
    passwords: Vec<Pwd>,
}

#[get("/generate_passwords")]
pub async fn generate_passwords(
    password_attribute: &State<PasswordAttributes>,
) -> Result<PasswordsTemplate, NotFound<String>> {
    let count = password_attribute.count.load(Ordering::Relaxed) as usize;
    let length = password_attribute.length.load(Ordering::Relaxed) as usize;

    let pg = PasswordGenerator {
        length,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    let mut pwd: Vec<Pwd> = Vec::new();
    pwd.reserve_exact(count);
    pg.generate(count)
        .unwrap()
        .into_iter()
        .map(|x| {
            pwd.push(Pwd {
                password: x.clone(),
                score: scorer::score(&analyzer::analyze(&x)).round() as u8,
            });
        })
        .count();

    let template = PasswordsTemplate { passwords: pwd };

    Ok(template)
}
