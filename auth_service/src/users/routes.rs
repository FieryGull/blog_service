use crate::{
    jwt_auth::{JwtMiddleware, create_jwt_token},
    users::model::{LoginUserSchema, RegisterUserSchema, User, FilteredUser},
    users::basic_auth::{verify},
    error_handler::CustomError,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpResponse,
};
use uuid::Uuid;
use serde_json::json;
use std::env;


#[post("/auth/register")]
async fn register(user: web::Json<RegisterUserSchema>) -> Result<HttpResponse, CustomError> {
    let user = User::create(User::from(user.into_inner()))?;
    Ok(HttpResponse::Ok().json(FilteredUser::from(user)))
}

#[post("/auth/login")]
async fn login(login_data: web::Json<LoginUserSchema>) -> Result<HttpResponse, CustomError> {
    let db_user = User::find_by_email(&login_data.email);
    let is_valid = db_user.as_ref().map_or(false, |user| verify(&login_data.password, &user.password));

    if !is_valid {
        return Err {
            0: CustomError::new(401,
                                "Invalid email or password".to_string())
        };
    }
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not found.");
    let token = create_jwt_token(jwt_secret, db_user.as_ref().unwrap().id.to_string());

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).json(json!({"token": token})))
}

#[get("/auth/logout")]
async fn logout(_: JwtMiddleware) -> Result<HttpResponse, CustomError> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    Ok(HttpResponse::Ok().cookie(cookie).finish())
}

#[get("/users")]
async fn find_all(_: JwtMiddleware) -> Result<HttpResponse, CustomError> {
    let users = web::block(|| User::find_all()).await.unwrap();
    let users = users.unwrap().into_iter()
        .map(|user| FilteredUser::from(user))
        .collect::<Vec<FilteredUser>>();
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>,
              _: JwtMiddleware) -> Result<HttpResponse, CustomError> {
    let user = User::find_by_id(id.into_inner())?;
    Ok(HttpResponse::Ok().json(FilteredUser::from(user)))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(register);
    config.service(login);
    config.service(logout);
}