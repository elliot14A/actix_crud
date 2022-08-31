use sea_orm::{ DatabaseConnection, EntityTrait};


use crate::{api::error::ServiceError, models::CreatePostResponse};
use crate::entities::{prelude::*, *};

pub async fn create_new_post(
    name: String,
    description: Option<String>,
    image_url: Option<String>,
    user_id: i32,
    db: &DatabaseConnection
) -> Result<CreatePostResponse, ServiceError> {
    let post = post::ActiveModel {
        name: sea_orm::ActiveValue::Set(name),
        image_url: sea_orm::ActiveValue::Set(image_url.into()),
        description: sea_orm::ActiveValue::Set(description.into()),
        user_id: sea_orm::ActiveValue::Set(user_id),
        ..Default::default()
    };

    Post::insert(post).exec_with_returning(db)
        .await
        .map(|v| CreatePostResponse {id: v.id, name: v.name, description: v.description, image_url: v.image_url})
        .map_err(|e| ServiceError::DBQueryError(format!("{:?}", e)))
    
}
