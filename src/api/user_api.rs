use crate::api::error::ServiceError;
use crate::models::{CreateUserModel, CreateUserResponse, LoginUserRequest, LoginUserResponse};
use crate::repositories::user_repository::{create_user_repository, login_user_repository};
use crate::AppData;
use actix_web::{
    post,
    web::{self, Json},
};
use actix_web_validator::Json as J;

#[post("/register")]
pub async fn create_user_handler(
    db: web::Data<AppData>,
    body: J<CreateUserModel>,
) -> Result<Json<CreateUserResponse>, ServiceError> {
    let CreateUserModel {
        email,
        password,
        name,
    } = body.into_inner();

    create_user_repository(name, email, password, &db.conn)
        .await
        .map(|v| Json(v))
}

#[post("/login")]
pub async fn login_user_handler(
    db: web::Data<AppData>,
    body: J<LoginUserRequest>,
) -> Result<Json<LoginUserResponse>, ServiceError> {
    let LoginUserRequest { email, password } = body.into_inner();
    login_user_repository(email, password, &db.conn)
        .await
        .map(|v| Json(v))
}
