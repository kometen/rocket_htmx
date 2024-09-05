use crate::PasswordAttributes;
use askama::Template;
use rocket::response::status::NotFound;
use rocket::State;
use std::sync::atomic::Ordering;

// Macro to handle GET and POST of password-options.
// The macro takes the following arguments:
// 1. The name of the template struct.
// 2. The path to the template file.
// 3. The name of the checkbox field in the template.
// 4. The REST endpoint for the GET and POST requests.
// 5. The function name for the GET request.
// 6. The function name for the POST request.
// 7. The field name in the PasswordAttributes struct to access the option state.
// 8. The function name to change the option state in the PasswordAttributes struct.
// The macro creates the following functions:
// 1. A GET request function that returns the template struct with the checkbox status.
// 2. A POST request function that changes the option state in the PasswordAttributes struct and returns the template struct with the updated checkbox status.
// The macro uses the provided arguments to construct the template struct, the GET endpoint, and the POST endpoint.

macro_rules! create_password_option_template {
    (
        $template_struct_name:ident,
        $template_path:literal,
        $checkbox_field_name:ident,
        $rest_endpoint:literal,
        $get_function_name:ident,
        $option_field_name:ident,
        $post_function_name:ident
    ) => {
        #[derive(Template)]
        #[template(path = $template_path)]
        pub struct $template_struct_name {
            $checkbox_field_name: String,
        }

        #[get($rest_endpoint)]
        pub async fn $get_function_name(
            password_attributes: &State<PasswordAttributes>,
        ) -> Result<$template_struct_name, NotFound<String>> {
            let checkbox_status = match password_attributes.$option_field_name.load(Ordering::Relaxed) {
                true => "checked".to_string(),
                false => "".to_string(),
            };

            let template = $template_struct_name {
                $checkbox_field_name: checkbox_status,
            };

            let response = template;
            Ok(response)
        }

        #[post($rest_endpoint)]
        pub async fn $post_function_name(
            password_attributes: &State<PasswordAttributes>,
        ) -> Result<$template_struct_name, NotFound<String>> {
            let checkbox_status = match password_attributes.$option_field_name.load(Ordering::Relaxed)
                && check_options(password_attributes)
            {
                true => {
                    password_attributes.$option_field_name.store(false, Ordering::Relaxed);
                    "".to_string()
                }
                false => {
                    password_attributes.$option_field_name.store(true, Ordering::Relaxed);
                    "checked".to_string()
                }
            };

            let template = $template_struct_name {
                $checkbox_field_name: checkbox_status,
            };

            let response = template;
            Ok(response)
        }
    };
}

create_password_option_template!(
    PasswordOptionNumbersTemplate,
    "components/password_option_numbers.html",
    numbers_checkbox,
    "/password_options/numbers",
    numbers_option,
    numbers,
    change_numbers_option
);

create_password_option_template!(
    PasswordOptionLowerCaseLettersTemplate,
    "components/password_option_lowercase_letters.html",
    lowercase_letters_checkbox,
    "/password_options/lowercase_letters",
    lowercase_letters_option,
    lowercase_letters,
    change_lowercase_letters_option
);

create_password_option_template!(
    PasswordOptionUpperCaseLettersTemplate,
    "components/password_option_uppercase_letters.html",
    uppercase_letters_checkbox,
    "/password_options/uppercase_letters",
    uppercase_letters_option,
    uppercase_letters,
    change_uppercase_letters_option
);

create_password_option_template!(
    PasswordOptionSymbolsTemplate,
    "components/password_option_symbols.html",
    symbols_checkbox,
    "/password_options/symbols",
    symbols_option,
    symbols,
    change_symbols_option
);

create_password_option_template!(
    PasswordOptionSpacesTemplate,
    "components/password_option_spaces.html",
    spaces_checkbox,
    "/password_options/spaces",
    spaces_option,
    spaces,
    change_spaces_option
);

create_password_option_template!(
    PasswordOptionExcludeSimilarCharactersTemplate,
    "components/password_option_exclude_similar_characters.html",
    exclude_similar_characters_checkbox,
    "/password_options/exclude_similar_characters",
    exclude_similar_characters_option,
    exclude_similar_characters,
    change_exclude_similar_characters_option
);

fn check_options(password_attributes: &State<PasswordAttributes>) -> bool {
    [
        password_attributes.numbers.load(Ordering::Relaxed),
        password_attributes
            .lowercase_letters
            .load(Ordering::Relaxed),
        password_attributes
            .uppercase_letters
            .load(Ordering::Relaxed),
        password_attributes.symbols.load(Ordering::Relaxed),
        password_attributes.spaces.load(Ordering::Relaxed),
    ]
    .into_iter()
    .filter(|&x| x)
    .count()
        > 1
}
