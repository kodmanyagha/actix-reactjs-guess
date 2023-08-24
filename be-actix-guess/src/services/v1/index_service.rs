use std::sync::Arc;

use actix_web::HttpRequest;

use crate::{
    http::middlewares::auth_middleware_fn::get_auth_header,
    models::{app_state::AppState, user::User},
    services::auth_service::get_user_from_token,
};

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum AppDataError {
    #[error("General error.")]
    GeneralError,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppDataResponse {
    token: String,
    user: User,
}

pub async fn get_app_data(
    http_req: HttpRequest,
    app_state: Arc<AppState>,
) -> anyhow::Result<AppDataResponse> {
    let token_str = get_auth_header(http_req.headers())?;
    let user = get_user_from_token(app_state, token_str)?;

    Ok(AppDataResponse {
        token: token_str.to_owned(),
        user: user.clone(),
    })
}
