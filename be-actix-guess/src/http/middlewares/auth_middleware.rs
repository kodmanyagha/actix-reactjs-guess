use actix_http::{body::MessageBody, header::HeaderMap, HttpMessage};
use actix_web::{dev, Error};
use actix_web_lab::middleware::Next;
use thiserror::Error;

use crate::models::app_state::{AppState, Token};

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
    GeneralError(String),
}

pub async fn auth_middleware(
    req: dev::ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<dev::ServiceResponse<impl MessageBody>, Error> {
    let token_result = find_token_from_request(&req);

    if let Ok(token) = token_result {
        println!("Auth middleware found token: {:?}", token);

        req.extensions_mut().insert(token);
    } else if let Err(err) = token_result {
        if let HeaderValidationError::GeneralError(description) = &err {
            println!("General error description: {description}");
        }

        println!("Token result err: {:?}", err);
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

pub fn find_token_from_request(
    req: &dev::ServiceRequest,
) -> anyhow::Result<Token, HeaderValidationError> {
    let state: &AppState =
        req.app_data::<AppState>()
            .ok_or(HeaderValidationError::GeneralError(
                "app_data is not reachable.".to_owned(),
            ))?;

    let header_str: &str = get_auth_header(req.headers())?;

    let tokens = state.tokens.lock().unwrap();

    if let Some(token) = tokens.get(header_str) {
        // TODO Check validation date: `token.valid_until`
        Ok(token.clone())
    } else {
        Err(HeaderValidationError::TokenExpiredError)
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

    println!("get_auth_header header_str: {:?}", header_str);

    if header_str.is_empty() {
        Err(HeaderValidationError::AuthHeaderCanNotBeEmpty)
    } else {
        Ok(header_str)
    }
}
