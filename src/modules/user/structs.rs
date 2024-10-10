use crate::models::user::UserStatus;

pub struct CreateUserProps {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_verified: bool,
    pub status: UserStatus,
}
