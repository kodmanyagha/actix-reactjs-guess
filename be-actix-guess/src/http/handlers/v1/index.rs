use actix_http::HttpMessage;
use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::{
    models::app_state::{AppState, Token},
    response::{GenericResponse, SingleResponse},
    services::v1::index_service,
};

#[get("/healthcheck")]
pub async fn health_check_handler() -> HttpResponse {
    let response_json = GenericResponse {
        status: "success".to_owned(),
        message: "success".to_owned(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/appData")]
pub async fn app_data_handler(
    app_state: web::Data<AppState>,
    http_req: HttpRequest,
) -> HttpResponse {
    let http_req_clone = http_req.clone();
    let extensions = http_req_clone.extensions();
    let found_token = extensions.get::<Token>();
    println!("found_token: {found_token:?}");

    let app_data = index_service::get_app_data(http_req.clone(), app_state.into_inner()).await;

    if let Ok(app_data) = app_data {
        HttpResponse::Ok().json(SingleResponse {
            status: "success".to_owned(),
            data: app_data,
        })
    } else {
        HttpResponse::Ok().json(SingleResponse::<Option<()>> {
            status: "success".to_owned(),
            data: None,
        })
    }
}

pub fn add_routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
        .service(health_check_handler)
        .service(app_data_handler);

    conf.service(scope);
}
