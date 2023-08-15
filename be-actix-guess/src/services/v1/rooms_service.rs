use chrono::Utc;
use std::sync::Arc;
use thiserror::Error;

use actix_web::HttpRequest;
use anyhow::Result;
use uuid::Uuid;

use crate::{
    http::{requests::v1::rooms::CreateGuessRoomRequest, utils::get_auth_user_id},
    models::{
        app_state::{AppState, PaginationRequest},
        guess::GuessRoom,
    },
    response::PaginationResponse,
};

pub async fn rooms_list(
    request: PaginationRequest,
    state: Arc<AppState>,
) -> Result<PaginationResponse<GuessRoom>> {
    let rooms = state.rooms.lock().unwrap();

    let limit = request.limit.unwrap_or(10);
    let offset = (request.page.unwrap_or(1) - 1) * limit;

    let rooms_page: Vec<GuessRoom> = rooms.clone().into_iter().skip(offset).take(limit).collect();

    Ok(PaginationResponse {
        status: "success".to_string(),
        recordsTotal: rooms.len() as u32,
        recordsFiltered: rooms.len() as u32,
        data: rooms_page,
    })
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CreateRoomError {
    #[error("Room can not be created.")]
    RoomCannotBeCreated,
}

pub async fn create_room(
    request: CreateGuessRoomRequest,
    http_req: HttpRequest,
    state: Arc<AppState>,
) -> Result<GuessRoom> {
    let mut rooms = state.rooms.lock().unwrap();
    let tokens = state.tokens.lock().unwrap();
    let users = state.users.lock().unwrap();
    let auth_user_id = get_auth_user_id(&http_req, &tokens);

    if auth_user_id.is_err() {
        return Err(CreateRoomError::RoomCannotBeCreated.into());
    }

    let auth_user = users
        .iter()
        .find(|u| u.id == *auth_user_id.as_ref().unwrap());

    if auth_user.is_none() {
        return Err(CreateRoomError::RoomCannotBeCreated.into());
    }

    let created_room = GuessRoom {
        id: (rooms.len() + 1) as u64,
        room_key: Uuid::new_v4().to_string(),
        creator_user_id: auth_user_id.unwrap(),
        answers: vec![],
        title: request.title,
        range: (request.min, request.max),
        max_user_count: request.max_user_count,
        completed: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    rooms.push(created_room.clone());

    Ok(created_room)
}

mod tests {
    #[test]
    fn test_create_room_success() {
        // TODO Handle here.
        let b = "test".to_owned();
        assert_eq!(b, String::from("test"));
    }
}
