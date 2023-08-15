use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    models::app_state::AppState,
    response::GenericResponse,
    services::auth_service::{self, RegisterError},
};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[post("/auth/login")]
async fn login_handler(
    req_data: web::Json<LoginRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let result = auth_service::login(req_data.into_inner(), state.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => HttpResponse::BadRequest().json(GenericResponse {
            status: "fail".to_string(),
            message: error.to_string(),
        }),
    }
}

#[post("/auth/register")]
async fn register_handler(
    req_data: web::Json<RegisterRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    println!(">> register_handler çağırıldı");

    let result = auth_service::register(req_data.into_inner(), state.into_inner()).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(error) => match error {
            RegisterError::UsernameExistError => HttpResponse::BadRequest().json(GenericResponse {
                status: "fail".to_string(),
                message: "Username already registered.".to_string(),
            }),
            RegisterError::PasswordLengthError => {
                HttpResponse::BadRequest().json(GenericResponse {
                    status: "fail".to_string(),
                    message: "Password length must be 6 character.".to_string(),
                })
            }
            RegisterError::OtherError => HttpResponse::BadRequest().json(GenericResponse {
                status: "fail".to_string(),
                message: "Unknown error.".to_string(),
            }),
        },
    }
}

pub fn add_routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
        .service(login_handler)
        .service(register_handler);

    conf.service(scope);
}
