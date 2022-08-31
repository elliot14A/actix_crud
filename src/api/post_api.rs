use actix_web::{
    web::{self, Json},
    HttpRequest,
};
use actix_web_validator::Json as J;

use crate::{
    models::{CreatePostRequest, CreatePostResponse},
    repositories::{post_repository::create_new_post, user_repository::valid_token},
    AppData,
};

use super::error::ServiceError;

pub async fn create_post_handler(
    db: web::Data<AppData>,
    body: J<CreatePostRequest>,
    req: HttpRequest,
) -> Result<Json<CreatePostResponse>, ServiceError> {
    let CreatePostRequest {
        name,
        description,
        image_url,
    } = body.into_inner();

    let authorization = req.headers().get("Authorization");

    if let None = authorization {
        return Err(ServiceError::Forbidden);
    }

    let token = authorization
        .unwrap()
        .to_str()
        .unwrap()
        .split("Bearer")
        .last();
    if let None = token {
        return Err(ServiceError::Forbidden);
    }

    let valid = valid_token(token.unwrap(), &db.conn).await;

    println!("{}", valid);

    if valid {
        return create_new_post(name, description, image_url, 1, &db.conn)
            .await
            .map(|v| Json(v));
    }

    Err(ServiceError::Forbidden)
}
