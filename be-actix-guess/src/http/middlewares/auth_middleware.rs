use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::Arc,
};

use actix_http::{h1, header::HeaderMap};
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error,
};
use futures::future::LocalBoxFuture;
use thiserror::Error;

use crate::models::app_state::AppState;

pub struct Logging;

impl<S: 'static, B> Transform<S, ServiceRequest> for Logging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            // TODO: Handle here.
            //let x = validate_auth_header(req.headers(), &state.into_inner());
            //if let Ok(user_id) = x {
            //    req.extensions_mut().insert(user_id);
            //}

            // extract bytes from request body
            let body = req.extract::<web::Bytes>().await.unwrap();
            println!("request body (middleware): {body:?}");

            // re-insert body back into request to be used by handlers
            req.set_payload(bytes_to_payload(body));

            let res = svc.call(req).await?;

            println!("response: {:?}", res.headers());
            Ok(res)
        })
    }
}

fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut pl) = h1::Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}

#[non_exhaustive]
#[derive(Debug, Error)]
enum HeaderValidationError {
    #[error("Authorization header not found.")]
    AuthHeaderNotFound,
    #[error("Token expired or not valid.")]
    TokenExpiredError,
    #[error("General authentication error.")]
    GeneralError,
}

fn validate_auth_header(headers: &HeaderMap, state: &Arc<AppState>) -> anyhow::Result<u64> {
    let header = headers
        .get("authorization")
        .ok_or(HeaderValidationError::AuthHeaderNotFound)?;
    let header_str = header.to_str()?.trim();

    println!("header_str: {:?}", header_str);

    if header_str.is_empty() {
        Err(HeaderValidationError::GeneralError.into())
    } else {
        let header_str = header.to_str().unwrap_or("");
        let tokens = state.tokens.lock().unwrap();

        if let Some(token) = tokens.get(header_str) {
            // TODO Check validation date: `token.valid_until`
            Ok(token.user_id)
        } else {
            Err(HeaderValidationError::TokenExpiredError.into())
        }
    }
}
