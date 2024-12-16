/*Process the request (checkstatus) into the service functions,
returns a response based on the service's outcome */

use crate::services::presence_service;
use crate::state::SharedState;
use actix_web::{web, HttpResponse};

pub async fn check_status(state: web::Data<SharedState>, info: web::Json<String>) -> HttpResponse {
    let user_id = info.into_inner();
    match presence_service::check_status(state.get_ref().clone(), &user_id) {
        Ok(status) => HttpResponse::Ok().body(status),
        Err(err) => HttpResponse::BadGateway().body(err),
    }
}

pub async fn send_message(state: web::Data<SharedState>, info: web::Json<(String, String)>) -> HttpResponse {
    let (receiver, message) = info.into_inner();
    match presence_service::send_message(state.get_ref().clone(), &receiver, &message).await{
        Ok(status) => HttpResponse::Ok().body(status),
        Err(err) => HttpResponse::BadGateway().body(err),
    }
}
