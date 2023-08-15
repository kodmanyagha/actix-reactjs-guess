use actix_web::{get, web, HttpResponse};

use crate::response::GenericResponse;

#[get("/healthcheck")]
pub async fn health_check_handler() -> HttpResponse {
    let response_json = GenericResponse {
        status: "success".to_owned(),
        message: "success".to_owned(),
    };
    HttpResponse::Ok().json(response_json)
}

pub fn add_routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(health_check_handler);

    conf.service(scope);
}
