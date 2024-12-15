/*Process the requests (register, login, logout) into the service functions,
returns a response based on the service's outcome */

use crate::services::auth_service;
use crate::state::SharedState;
use actix_web::{web, HttpResponse, HttpRequest};
use actix_ws::AggregatedMessage;
use actix::prelude::*;
use actix_ws::Session;

/* 
pub async fn register(
    state: web::Data<SharedState>,
    info: web::Json<(String, String)>, // (user_id,password)
) -> HttpResponse {
    let (user_id, password) = info.into_inner();
    match auth_service::register(session, state.get_ref().clone(), &user_id, &password) {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
*/
pub async fn login(
    session:Session, 
    req: HttpRequest, 
    state: web::Data<SharedState>,
    info: web::Json<(String, String)>, // (user_id,password)
) -> HttpResponse {
    let (user_id, password) = info.into_inner();
    match auth_service::login(session, &req,state.get_ref().clone(), &user_id, &password) {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

pub async fn logout(
    state: web::Data<SharedState>,
    info: web::Json<String>, // user_id
) -> HttpResponse {
    let user_id = info.into_inner();
    match auth_service::logout(state.get_ref().clone(), &user_id) {
        Ok(msg) => HttpResponse::Ok().body(msg),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
