use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth_user")
            .route(web::post().to(auth_user_endpoint))
    );
}



#[derive(Deserialize)]
pub struct AuthUserData {
    username: String,
    password: String
}


async fn auth_user_endpoint(user_data: web::Json<AuthUserData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Auth user with username {} and pass {}", user_data.username, user_data.password))
}