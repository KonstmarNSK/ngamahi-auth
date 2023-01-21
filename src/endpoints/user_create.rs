use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/create_user")
            .route(web::post().to(create_user_endpoint))
    );
}



#[derive(Deserialize)]
pub struct CreateUserData {
    username: String,
    password: String
}


async fn create_user_endpoint(user_data: web::Json<CreateUserData>) -> impl Responder {
    HttpResponse::Ok().body(format!("Got user info with username {} and pass {}", user_data.username, user_data.password))
}