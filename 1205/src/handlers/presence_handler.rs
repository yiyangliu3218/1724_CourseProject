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
