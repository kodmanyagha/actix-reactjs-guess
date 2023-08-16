use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct User {
    pub id: u64,
    pub username: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub gender: Option<Gender>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl ToString for Gender {
    fn to_string(&self) -> String {
        match self {
            Gender::Male => String::from("male"),
            Gender::Female => String::from("female"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct JwtClaim {
    pub username: String,
    pub iss: String,
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

impl JwtClaim {
    pub fn new(user_id: u64, username: String, expire: u64) -> Self {
        JwtClaim {
            username: username.clone(),
            iss: "actix".to_string(),
            sub: user_id.clone().to_string(),
            iat: Utc::now().timestamp() as u64,
            exp: expire.clone(),
        }
    }
}
