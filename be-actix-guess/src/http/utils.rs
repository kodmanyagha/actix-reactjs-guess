use std::collections::HashMap;

use actix_web::HttpRequest;

use crate::models::app_state::Token;

#[derive(Debug)]
pub enum AuthError {
    HeaderNotProvided,
    TokenExpired,
    TokenNotFound,
    TokenNotProvided,
}

// TODO: Move this algorithm in a middleware.
pub fn get_auth_user_id(
    req: &HttpRequest,
    tokens: &HashMap<String, Token>,
) -> Result<u64, AuthError> {
    let auth_header = req.headers().get("authorization");

    if auth_header.is_none() {
        return Err(AuthError::HeaderNotProvided);
    } else {
        let token_val = auth_header.unwrap().to_str();
        if token_val.is_ok() {
            let user_id_attempt = tokens.get(&token_val.unwrap().to_owned());

            if user_id_attempt.is_some() {
                return Ok(user_id_attempt.unwrap().user_id.clone());
            } else {
                return Err(AuthError::TokenNotFound);
            }
        } else {
            return Err(AuthError::TokenNotProvided);
        }
    }
}
