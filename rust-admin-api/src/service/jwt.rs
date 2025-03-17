use std:: time::{Duration, SystemTime, UNIX_EPOCH};
use dotenv::dotenv;
use jsonwebtoken:: {encode, Algorithm, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::entity::system_user::SystemUser;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginClaims {
    pub user: ClaimUser,
    pub exp: usize
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct ClaimUser {
    pub user_id: i32,
    pub user_name: String,
    pub real_name: String,
    pub is_admin: i8,
}

impl From<SystemUser> for ClaimUser {
    fn from(user: SystemUser) -> Self {
        Self {
            user_id: user.id,
            user_name: user.user_name,
            real_name: user.real_name,
            is_admin: user.is_admin,    
        }
    }
}

pub struct JwtService;

impl JwtService {
    pub fn create_login_token(user: ClaimUser) -> String {
        dotenv().ok();
        let secret = std::env::var("JWT_SECRET").expect("DATABASE_URL must be set");
        let claims = LoginClaims {
            user: user.into(),
            exp: (SystemTime::now() + Duration::from_secs(60 * 60 * 24)).duration_since(UNIX_EPOCH).unwrap()
            .as_secs() as usize
        };
        let token = encode(&Header::new(Algorithm::HS256), &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_ref())).unwrap();
        token
    }
    pub fn verify_login_token(token: &str) -> Result<LoginClaims, jsonwebtoken::errors::Error> {
        //替换掉字符串头部的Bearer
        let token = token.to_owned().replace("Bearer ", "");
        dotenv().ok();
        let secret = std::env::var("JWT_SECRET").expect("DATABASE_URL must be set");
        let token_data = jsonwebtoken::decode::<LoginClaims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256));
        match token_data {
            Ok(data) => Ok(data.claims),
            Err(e) => Err(e)
        }
    }
}