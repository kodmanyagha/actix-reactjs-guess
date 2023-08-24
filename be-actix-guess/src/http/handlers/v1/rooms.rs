use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
    http::requests::v1::rooms::CreateGuessRoomRequest,
    models::app_state::{AppState, PaginationRequest},
    services::v1::rooms_service,
};

#[get("/")]
pub async fn rooms_list_handler(
    req_data: web::Query<PaginationRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    let result = rooms_service::rooms_list(req_data.into_inner(), state.into_inner()).await;

    if let Ok(d) = result {
        HttpResponse::Ok().json(d)
    } else {
        HttpResponse::Ok().json("An error occured.")
    }
}

#[post("/")]
async fn create_room_handler(
    req_data: web::Json<CreateGuessRoomRequest>,
    http_req: HttpRequest,
    state: web::Data<AppState>,
) -> impl Responder {
    let result =
        rooms_service::create_room(req_data.into_inner(), http_req, state.into_inner()).await;

    if let Ok(d) = result {
        HttpResponse::Ok().json(d)
    } else {
        HttpResponse::Ok().json("An error occured.")
    }
}

pub fn add_routes(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1/rooms")
        .service(rooms_list_handler)
        .service(create_room_handler);

    conf.service(scope);
}
