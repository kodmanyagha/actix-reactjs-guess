use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GuessRoom {
    pub id: u64,
    pub room_key: String,
    pub creator_user_id: u64,
    pub answers: Vec<Answer>,
    pub title: String,
    pub range: (i32, i32),
    pub max_user_count: u64,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Answer {
    pub user_id: u64,
    pub answer: Option<i32>,
    pub is_correct: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
