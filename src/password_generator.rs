use crate::PasswordAttributes;
use passwords::{analyzer, scorer, PasswordGenerator};
use rocket::serde::json::Json;
use rocket::State;
use std::sync::atomic::Ordering;

#[derive(serde::Serialize, Debug)]
pub struct Pwd {
    password: String,
    score: f64,
}

pub fn produce_passwords(password_attribute: &State<PasswordAttributes>) -> Json<Vec<Pwd>> {
    let count = password_attribute.count.load(Ordering::Relaxed) as usize;
    let length = password_attribute.length.load(Ordering::Relaxed) as usize;

    let pg = PasswordGenerator {
        length,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };

    let mut pwd: Vec<Pwd> = Vec::with_capacity(count);
    pg.generate(count)
        .unwrap()
        .into_iter()
        .map(|x| {
            pwd.push(Pwd {
                password: x.clone(),
                score: scorer::score(&analyzer::analyze(&x)),
            });
        })
        .count();
    Json(pwd)
}
