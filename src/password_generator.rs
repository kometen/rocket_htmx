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

    let pg = PasswordGenerator {
        length: password_attribute.length.load(Ordering::Relaxed) as usize,
        numbers: password_attribute.numbers.load(Ordering::Relaxed),
        lowercase_letters: password_attribute.lowercase_letters.load(Ordering::Relaxed),
        uppercase_letters: password_attribute.uppercase_letters.load(Ordering::Relaxed),
        symbols: password_attribute.symbols.load(Ordering::Relaxed),
        spaces: password_attribute.spaces.load(Ordering::Relaxed),
        exclude_similar_characters: password_attribute
            .exclude_similar_characters
            .load(Ordering::Relaxed),
        strict: password_attribute.strict.load(Ordering::Relaxed),
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
