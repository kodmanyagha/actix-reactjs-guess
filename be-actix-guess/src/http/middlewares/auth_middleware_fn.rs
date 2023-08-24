use std::sync::Arc;

use actix_http::{body::MessageBody, header::HeaderMap, HttpMessage};
use actix_web::{dev, web, Error, Responder};
use actix_web_lab::middleware::Next;
use thiserror::Error;

use crate::models::app_state::AppState;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum HeaderValidationError {
    #[error("Authorization header not found.")]
    AuthHeaderNotFound,

    #[error("Authorization header can not be empty.")]
    AuthHeaderCanNotBeEmpty,

    #[error("Token expired or not valid.")]
    TokenExpiredError,

    #[error("General authentication error.")]
    GeneralError,
}

pub async fn auth_middleware(
    req: dev::ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<dev::ServiceResponse<impl MessageBody>, Error> {
    println!(">> auth_middleware called.");

    let x = validate_auth_header(&req);
    if let Ok(user_id) = x {
        println!("Found user id: {user_id}");
        req.extensions_mut().insert(user_id);
    } else if let Err(err) = x {
        println!("err: {:?}", err);
    }

    // this is working:
    Ok(next.call(req).await?)

    //    let x = next.call(req).await?;
    //    match next.call(req).await? {
    //        Ok(res) => match res {
    //            Ok(res) => res,
    //            Err(_err) => Err(actix_web::error::ErrorUnauthorized("")),
    //        },
    //        Err(_err) => Err(actix_web::error::ErrorUnauthorized("")),
    //    }
}

pub fn validate_auth_header(
    req: &dev::ServiceRequest,
) -> anyhow::Result<u64, HeaderValidationError> {
    let state: &AppState = req
        .app_data::<AppState>()
        .ok_or(HeaderValidationError::GeneralError)?;

    let header_str: &str = get_auth_header(req.headers())?;

    println!("header_str: {:?}", header_str);

    if header_str.is_empty() {
        Err(HeaderValidationError::GeneralError)
    } else {
        let tokens = state.tokens.lock().unwrap();

        if let Some(token) = tokens.get(header_str) {
            // TODO Check validation date: `token.valid_until`
            Ok(token.user_id)
        } else {
            Err(HeaderValidationError::TokenExpiredError.into())
        }
    }
}

pub fn get_auth_header(headers: &HeaderMap) -> anyhow::Result<&str, HeaderValidationError> {
    let header = headers
        .get("authorization")
        .ok_or(HeaderValidationError::AuthHeaderNotFound)?;

    let header_str: &str = header
        .to_str()
        .ok()
        .ok_or(HeaderValidationError::AuthHeaderNotFound)?
        .trim();

    println!("header_str: {:?}", header_str);

    if header_str.is_empty() {
        Err(HeaderValidationError::GeneralError)
    } else {
        Ok(header_str)
    }
}
