
// use actix_web::{web, HttpResponse};
// use crate::services::auth_service;
// use crate::state::SharedState;
// use serde::Deserialize;

// #[derive(Deserialize)]
// pub struct RegisterRequest {
//     pub user_id: String,
//     pub password: String,
// }

// #[derive(Deserialize)]
// pub struct LoginRequest {
//     pub user_id: String,
//     pub password: String,
// }

//     pub async fn register(
//         state: web::Data<SharedState>,
//         req: web::Json<RegisterRequest>,
//     ) -> HttpResponse {
//         let register_data = req.into_inner();
//         match auth_service::register(
//             state.get_ref().clone(),
//             &register_data.user_id,
//             &register_data.password,
//         ) {
//             Ok(msg) => HttpResponse::Ok().body(msg),
//             Err(err) => HttpResponse::BadRequest().body(err),
//         }
//     }

//     pub async fn login(
//         state: web::Data<SharedState>,
//         req: web::Json<LoginRequest>,
//     ) -> HttpResponse {
//         let login_data = req.into_inner();
//         match auth_service::login(
//             state.get_ref().clone(),
//             &login_data.user_id,
//             &login_data.password,
//         ) {
//             Ok(msg) => HttpResponse::Ok().body(msg),
//             Err(err) => HttpResponse::BadRequest().body(err),
//         }
//     }
use actix_web::{web, HttpResponse, Responder};
use crate::services::auth_service;
use crate::state::SharedState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub user_id: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub user_id: String,
    pub password: String,
}

pub async fn register(
    state: web::Data<SharedState>,
    data: web::Json<RegisterRequest>, // (user_id, password)
) -> impl Responder {
    let result: Result<String, String> = auth_service::register(
        state.get_ref().clone(),
        &data.user_id,
        &data.password,
    );
    match result {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn login(
    state: web::Data<SharedState>,
    data: web::Json<LoginRequest>, // (user_id, password)
) -> impl Responder {
    let result: Result<String, String> = auth_service::login(
        state.get_ref().clone(),
        &data.user_id,
        &data.password,
    );
    match result {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
