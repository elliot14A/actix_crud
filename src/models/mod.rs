use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserModel {
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginUserResponse {
    pub access_token: String,
}

#[derive(Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 5))]
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Serialize)]
pub struct CreatePostResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}
