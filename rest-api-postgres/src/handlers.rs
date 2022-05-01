use super::models::{NewUser, User};
use actix_web::{delete, error, get, post, web, Error, HttpResponse};
use uuid::Uuid;

#[get("/users")]
pub async fn get_all_users() -> Result<HttpResponse, Error> {
    User::find_all()
    .map(|u| HttpResponse::Ok().json(u))
    .map_err(|e| error::ErrorInternalServerError(e))
}

#[get("/users/{id}")]
pub async fn get_user(user_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    User::find(user_id.into_inner())
    .map(|u| HttpResponse::Ok().json(u))
    .map_err(|e| error::ErrorInternalServerError(e))
}

#[post("/users")]
pub async fn create_user(new_user: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    User::create(new_user.into_inner())
    .map(|u| HttpResponse::Created().json(u))
    .map_err(|e| error::ErrorInternalServerError(e))
}

#[delete("/users/{id}")]
pub async fn delete_user(user_id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    User::delete(user_id.into_inner())
    .map(|u| HttpResponse::Ok().json(u))
    .map_err(|e| error::ErrorInternalServerError(e))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(get_user);
    cfg.service(create_user);
    cfg.service(delete_user);
}
