use crate::routes::configure_routes;
use crate::state::AppState;
use actix_web::{web, App, HttpServer};

mod models;
mod routes;
mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
