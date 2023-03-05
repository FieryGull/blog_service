use crate::users::{User, RegisterUserSchema, LoginUserSchema};
use crate::error_handler::CustomError;
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;




#[post("/register")]
async fn register(user: web::Json<RegisterUserSchema>) -> Result<HttpResponse, CustomError> {
    let user = User::create(User::from(user.into_inner()))?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn login(user: web::Json<LoginUserSchema>) -> Result<HttpResponse, CustomError> {
    Ok(HttpResponse::Ok().json(user))
}

#[get("/users")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let users = web::block(|| User::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, CustomError> {
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(register);
    config.service(login);
}