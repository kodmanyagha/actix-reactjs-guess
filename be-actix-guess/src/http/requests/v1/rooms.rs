use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CreateGuessRoomRequest {
    pub title: String,
    pub min: i32,
    pub max: i32,
    pub max_user_count: u64,
}
