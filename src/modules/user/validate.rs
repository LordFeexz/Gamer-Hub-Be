use crate::helpers::validator::password_validation;
use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Debug, Deserialize)]
pub struct RegisterUser {
    #[validate(length(
        min = 3,
        max = 30,
        message = "Username must be between 3 and 30 characters"
    ))]
    pub username: String,

    #[validate(email(message = "invalid email format"))]
    pub email: String,

    #[validate(custom(function = "password_validation"))]
    pub password: String,
}
