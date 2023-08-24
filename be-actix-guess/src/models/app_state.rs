use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::models::{guess::GuessRoom, user::User};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Token {
    pub user_id: u64,
    pub valid_until: DateTime<Utc>,
}

impl Token {
    pub fn new(user_id: u64, valid_until: DateTime<Utc>) -> Self {
        Token {
            user_id,
            valid_until,
        }
    }
}

pub type TokenHashMap = HashMap<String, Token>;

#[derive(Default)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
    pub rooms: Arc<Mutex<Vec<GuessRoom>>>,
    pub tokens: Arc<Mutex<TokenHashMap>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
