use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use be_actix_guess::http::handlers::v1::*;
use be_actix_guess::http::middlewares::auth_middleware;
use be_actix_guess::models::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let server_host = "127.0.0.1";
    let server_port = 8001;

    let app_data = web::Data::new(AppState::new());

    println!(
        "🚀 Server started at http://{}:{}",
        server_host, server_port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:5173/")
            // https://stackoverflow.com/a/74867364/2132069
            //.allowed_origin("*")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .wrap(from_fn(auth_middleware::auth_middleware))
            //.wrap(auth_middleware::Logging)
            .wrap(cors)
            .wrap(Logger::default())
            .configure(|conf: &mut web::ServiceConfig| {
                auth::add_routes(conf);
                rooms::add_routes(conf);
                index::add_routes(conf);
            })
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
