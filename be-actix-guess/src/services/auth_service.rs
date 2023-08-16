use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    http::handlers::v1::auth::{LoginRequest, RegisterRequest},
    models::{
        app_state::{AppState, Token, TokenHashMap},
        user::User,
    },
    response::SingleResponse,
};

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LoginError {
    #[error("User not found.")]
    UserNotFound,
    #[error("Wrong credentials.")]
    WrongCredentials,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: String,
    user: User,
}

pub async fn login(body: LoginRequest, data: Arc<AppState>) -> Result<impl Serialize, LoginError> {
    let users = data.users.lock().unwrap();
    let mut tokens = data.tokens.lock().unwrap();

    let user = users.iter().find(|item| item.username == body.username);

    if user.is_none() {
        return Err(LoginError::UserNotFound);
    }

    let user = user.unwrap();

    if user.password != body.password {
        return Err(LoginError::WrongCredentials);
    }

    let hash = create_auth_token(&mut tokens, user.id, 8);

    Ok(SingleResponse {
        status: "success".to_string(),
        data: LoginResponse {
            token: hash,
            user: user.clone(),
        },
    })
}

pub fn create_auth_token(tokens: &mut TokenHashMap, user_id: u64, duration_hour: i64) -> String {
    let hash_result = sha256::digest(Alphanumeric.sample_string(&mut rand::thread_rng(), 16));
    tokens.insert(
        hash_result.clone(),
        Token::new(user_id, Utc::now() + Duration::hours(duration_hour)),
    );

    hash_result
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("This username already exist.")]
    UsernameExistError,

    #[error("Password must be minimum 6 character length.")]
    PasswordLengthError,

    #[error("Please check inputs.")]
    OtherError,
}

pub async fn register(
    body: RegisterRequest,
    data: Arc<AppState>,
) -> Result<impl Serialize, RegisterError> {
    let mut tokens = data.tokens.lock().unwrap();
    let mut users = data.users.lock().unwrap();
    let user = users.iter().find(|item| item.username == body.username);

    if user.is_some() {
        return Err(RegisterError::UsernameExistError);
    }

    if body.password.len() < 6 {
        return Err(RegisterError::PasswordLengthError);
    }

    let user = User {
        id: users
            .iter()
            .fold(0, |accumulator, item| accumulator.max(item.id))
            + 1,
        firstname: Some(body.username.clone()),
        lastname: None,
        gender: None,
        username: body.username.clone(),
        password: body.password.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    users.push(user.clone());

    let hash = create_auth_token(&mut tokens, user.id, 8);

    Ok(SingleResponse {
        status: "success".to_string(),
        data: LoginResponse {
            token: hash,
            user: user.clone(),
        },
    })
}
