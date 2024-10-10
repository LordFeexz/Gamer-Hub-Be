use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenDiscordData {
    pub id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_expires: u64,
    pub last_updated: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenValue {
    pub id: String,
    pub is_verified: bool,
    pub is_admin: bool,
    pub discord_data: Option<TokenDiscordData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    #[serde(flatten)]
    pub value: TokenValue,
    pub exp: usize,
}

pub struct Jwt {
    secret: String,
    admin_secret: String,
}

impl Jwt {
    pub fn new() -> Self {
        dotenv().ok();
        let secret = env::var("SECRET").expect("SECRET must be set");
        let admin_secret = env::var("ADMIN_SECRET").expect("ADMIN_SECRET must be set");

        Jwt {
            secret,
            admin_secret,
        }
    }

    // Fungsi untuk memverifikasi token
    pub fn verify_token(&self, token: &str) -> Result<TokenPayload, jsonwebtoken::errors::Error> {
        let token_data = decode::<TokenPayload>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }

    // Fungsi untuk membuat token
    pub fn create_token(
        &self,
        payload: TokenValue,
        exp: usize,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = TokenPayload {
            value: payload,
            exp,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        Ok(token)
    }

    // Fungsi untuk membuat token admin
    pub fn create_token_admin(
        &self,
        payload: &TokenValue,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let token = encode(
            &Header::new(Algorithm::HS256),
            &payload,
            &EncodingKey::from_secret(self.admin_secret.as_ref()),
        )?;
        Ok(token)
    }

    // Fungsi untuk memverifikasi token admin
    pub fn verify_admin_token(
        &self,
        token: &str,
    ) -> Result<TokenPayload, jsonwebtoken::errors::Error> {
        let token_data = decode::<TokenPayload>(
            token,
            &DecodingKey::from_secret(self.admin_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }
}
