use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use crate::common_types::tokens::RefreshToken;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/accept-refresh-token")
            .route(web::post().to(accept_refresh_token_endpoint))
    );
}



async fn accept_refresh_token_endpoint(user_data: web::Json<RefreshToken>) -> impl Responder {
    HttpResponse::Ok().body(format!("Auth user with username {} and pass {}", user_data.username, user_data.password))
}