use crate::PasswordAttributes;
use askama::Template;
use rocket::response::status::NotFound;
use rocket::State;
use std::sync::atomic::Ordering;

macro_rules! create_password_option_template {
    ($struct_name:ident, $template_path:literal, $checkbox_name:ident) => {
        #[derive(Template)]
        #[template(path = $template_path)]
        pub struct $struct_name {
            $checkbox_name: String,
        }
    };
}

create_password_option_template!(
    PasswordOptionNumbersTemplate,
    "components/password_option_numbers.html",
    numbers_checkbox
);

#[get("/password_options/numbers")]
pub async fn numbers_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionNumbersTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes.numbers.load(Ordering::Relaxed) {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let template = PasswordOptionNumbersTemplate {
        numbers_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

#[post("/password_options/numbers")]
pub async fn change_numbers_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionNumbersTemplate, NotFound<String>> {
    println!("check_options: {}", check_options(password_attributes));
    let checkbox_status = match password_attributes
        .numbers
        .load(Ordering::Relaxed)
        && check_options(password_attributes)
    {
        true => {
            password_attributes.numbers.store(false, Ordering::Relaxed);
            password_attributes.strict.store(false, Ordering::Relaxed);
            "".to_string()
        }
        false => {
            password_attributes.numbers.store(true, Ordering::Relaxed);
            "checked".to_string()
        }
    };

    let template = PasswordOptionNumbersTemplate {
        numbers_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

create_password_option_template!(
    PasswordOptionLowerCaseLettersTemplate,
    "components/password_option_lowercase_letters.html",
    lowercase_letters_checkbox
);

#[get("/password_options/lowercase_letters")]
pub async fn lowercase_letters_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionLowerCaseLettersTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .lowercase_letters
        .load(Ordering::Relaxed)
    {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let template = PasswordOptionLowerCaseLettersTemplate {
        lowercase_letters_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

#[post("/password_options/lowercase_letters")]
pub async fn change_lowercase_letters_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionLowerCaseLettersTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .lowercase_letters
        .load(Ordering::Relaxed)
        && check_options(password_attributes)
    {
        true => {
            password_attributes
                .lowercase_letters
                .store(false, Ordering::Relaxed);
            "".to_string()
        }
        false => {
            password_attributes
                .lowercase_letters
                .store(true, Ordering::Relaxed);
            "checked".to_string()
        }
    };

    let template = PasswordOptionLowerCaseLettersTemplate {
        lowercase_letters_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

create_password_option_template!(
    PasswordOptionUpperCaseLettersTemplate,
    "components/password_option_uppercase_letters.html",
    uppercase_letters_checkbox
);

#[get("/password_options/uppercase_letters")]
pub async fn uppercase_letters_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionUpperCaseLettersTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .uppercase_letters
        .load(Ordering::Relaxed)
    {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let template = PasswordOptionUpperCaseLettersTemplate {
        uppercase_letters_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

#[post("/password_options/uppercase_letters")]
pub async fn change_uppercase_letters_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionUpperCaseLettersTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .uppercase_letters
        .load(Ordering::Relaxed)
        && check_options(password_attributes)
    {
        true => {
            password_attributes
                .uppercase_letters
                .store(false, Ordering::Relaxed);
            "".to_string()
        }
        false => {
            password_attributes
                .uppercase_letters
                .store(true, Ordering::Relaxed);
            "checked".to_string()
        }
    };

    let template = PasswordOptionUpperCaseLettersTemplate {
        uppercase_letters_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

create_password_option_template!(
    PasswordOptionSymbolsTemplate,
    "components/password_option_symbols.html",
    symbols_checkbox
);

#[get("/password_options/symbols")]
pub async fn symbols_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionSymbolsTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes.symbols.load(Ordering::Relaxed) {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let template = PasswordOptionSymbolsTemplate {
        symbols_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

#[post("/password_options/symbols")]
pub async fn change_symbols_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionSymbolsTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .symbols
        .load(Ordering::Relaxed)
        && check_options(password_attributes)
    {
        true => {
            password_attributes.symbols.store(false, Ordering::Relaxed);
            "".to_string()
        }
        false => {
            password_attributes.symbols.store(true, Ordering::Relaxed);
            "checked".to_string()
        }
    };

    let template = PasswordOptionSymbolsTemplate {
        symbols_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

create_password_option_template!(
    PasswordOptionSpacesTemplate,
    "components/password_option_spaces.html",
    spaces_checkbox
);

#[get("/password_options/spaces")]
pub async fn spaces_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionSpacesTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes.spaces.load(Ordering::Relaxed) {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let template = PasswordOptionSpacesTemplate {
        spaces_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

#[post("/password_options/spaces")]
pub async fn change_spaces_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionSpacesTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .spaces
        .load(Ordering::Relaxed)
        && check_options(password_attributes)
    {
        true => {
            password_attributes.spaces.store(false, Ordering::Relaxed);
            "".to_string()
        }
        false => {
            password_attributes.spaces.store(true, Ordering::Relaxed);
            "checked".to_string()
        }
    };

    let template = PasswordOptionSpacesTemplate {
        spaces_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

create_password_option_template!(
    PasswordOptionExcludeSimilarCharactersTemplate,
    "components/password_option_exclude_similar_characters.html",
    exclude_similar_characters_checkbox
);

#[get("/password_options/exclude_similar_characters")]
pub async fn exclude_similar_characters_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionExcludeSimilarCharactersTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .exclude_similar_characters
        .load(Ordering::Relaxed)
    {
        true => "checked".to_string(),
        false => "".to_string(),
    };

    let template = PasswordOptionExcludeSimilarCharactersTemplate {
        exclude_similar_characters_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

#[post("/password_options/exclude_similar_characters")]
pub async fn change_exclude_similar_characters_option(
    password_attributes: &State<PasswordAttributes>,
) -> Result<PasswordOptionExcludeSimilarCharactersTemplate, NotFound<String>> {
    let checkbox_status = match password_attributes
        .exclude_similar_characters
        .load(Ordering::Relaxed)
    {
        true => {
            password_attributes
                .exclude_similar_characters
                .store(false, Ordering::Relaxed);
            "".to_string()
        }
        false => {
            password_attributes
                .exclude_similar_characters
                .store(true, Ordering::Relaxed);
            "checked".to_string()
        }
    };

    let template = PasswordOptionExcludeSimilarCharactersTemplate {
        exclude_similar_characters_checkbox: checkbox_status,
    };

    let response = template;
    Ok(response)
}

fn check_options(password_attributes: &State<PasswordAttributes>) -> bool {
    [
        password_attributes.numbers.load(Ordering::Relaxed),
        password_attributes.lowercase_letters.load(Ordering::Relaxed),
        password_attributes.uppercase_letters.load(Ordering::Relaxed),
        password_attributes.symbols.load(Ordering::Relaxed),
        password_attributes.spaces.load(Ordering::Relaxed),
    ]
        .into_iter()
        .filter(|&x| x)
        .count()
        > 1
}
