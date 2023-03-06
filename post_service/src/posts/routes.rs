use crate::{
    posts::{NewPost, Posts},
    common_lib::{
        error_handler::CustomError,
        jwt_auth::JwtMiddleware,
    },
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use uuid::Uuid;

#[get("/posts")]
async fn find_all(user_data: JwtMiddleware) -> Result<HttpResponse, CustomError> {
    let posts = web::block(move || Posts::find_all(user_data.user_id)).await.unwrap();
    Ok(HttpResponse::Ok().json(posts.unwrap()))
}

#[get("/posts/{id}")]
async fn find(id: web::Path<Uuid>,
              user_data: JwtMiddleware) -> Result<HttpResponse, CustomError> {
    let post = Posts::find(id.into_inner(), user_data.user_id)?;
    Ok(HttpResponse::Ok().json(post))
}

#[post("/posts")]
async fn create(post: web::Json<NewPost>,
                user_data: JwtMiddleware) -> Result<HttpResponse, CustomError> {
    let post = Posts::create(user_data.user_id, post.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[put("/posts/{id}")]
async fn update(
    id: web::Path<Uuid>,
    post: web::Json<NewPost>,
    user_data: JwtMiddleware,
) -> Result<HttpResponse, CustomError> {
    let post = Posts::update(id.into_inner(), user_data.user_id, post.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[delete("/posts/{id}")]
async fn delete(id: web::Path<Uuid>,
                user_data: JwtMiddleware) -> Result<HttpResponse, CustomError> {
    let deleted_post = Posts::delete(id.into_inner(), user_data.user_id)?;
    match deleted_post {
        0 => Err(CustomError::new(404, "The query object not found".to_string())),
        _ => Ok(HttpResponse::Ok().finish()),
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}