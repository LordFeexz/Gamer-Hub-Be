use std::borrow::Cow;

use regex::Regex;
use validator::ValidationError;

pub fn password_validation(password: &str) -> Result<(), ValidationError> {
    let requirements = vec![
        (
            r"[a-z]",
            "lower case",
            "password must contain at least 1 lower case",
        ),
        (
            r"[A-Z]",
            "upper case",
            "password must contain at least 1 upper case",
        ),
        (
            r"\d",
            "must contain number",
            "password must contain at least 1 number",
        ),
        (
            r"[!@#$%^&*]",
            "must contain symbol",
            "password must contain at least 1 symbol",
        ),
        (
            r"^.{8,}$",
            "at least 8 characters",
            "password must be at least 8 characters long",
        ),
    ];

    for (regex, code, message) in requirements {
        let re = Regex::new(regex).unwrap();
        if !re.is_match(password) {
            return Err(ValidationError::new(code).with_message(Cow::from(message)));
        }
    }

    Ok(())
}
