use crate::routes::configure_routes;
use crate::state::AppState;
use actix_web::{web, App, HttpServer};
use std::env;
// use std::sync::Arc;

mod cli;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let state = AppState::new();
    if args.len() > 1 && args[1] == "cli" {
        println!("Starting CLI...");
        cli::start_cli().await;
        return Ok(());
    } else {
        // Start the Actix Web server as before
        println!("Starting Actix Web server...");
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(state.clone()))
                .configure(configure_routes)
        })
        .bind("127.0.0.1:8081")?
        .run()
        .await
    }
}
