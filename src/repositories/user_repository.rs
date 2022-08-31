use crate::api::error::ServiceError;
use crate::entities::{prelude::*, *};
use crate::models::{CreateUserResponse, LoginUserResponse};

use argon2::Config;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn create_user_repository(
    name: String,
    email: String,
    password: String,
    conn: &DatabaseConnection,
) -> Result<CreateUserResponse, ServiceError> {
    let config = Config::default();
    let password = password.as_bytes();
    let salt = b"0x6c67c70047d49b48a9329ecb7c0e21e738dbe7d8702615af0b8180d0f6412135";
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    let user = user::ActiveModel {
        email: sea_orm::ActiveValue::Set(email),
        name: sea_orm::ActiveValue::Set(name),
        password: sea_orm::ActiveValue::Set(hash),
        ..Default::default()
    };

    let res = User::insert(user).exec_with_returning(conn).await;

    match res {
        Ok(v) => Ok(CreateUserResponse {
            id: v.id,
            name: v.name,
            email: v.email,
        }),
        Err(e) => Err(ServiceError::DBQueryError(format!("{:?}", e))),
    }
}

pub async fn login_user_repository(
    email: String,
    password: String,
    conn: &DatabaseConnection,
) -> Result<LoginUserResponse, ServiceError> {
    let user = User::find()
        .filter(user::Column::Email.eq(email))
        .one(conn)
        .await;

    if let Ok(v) = &user {
        match v {
            None => return Err(ServiceError::InvalidEmailOrPassword),
            Some(m) => {
                let mut config = Config::default();
                config.hash_length = 256;
                let salt = b"0x6c67c70047d49b48a9329ecb7c0e21e738dbe7d8702615af0b8180d0f6412135";
                let hash = argon2::hash_encoded("secret hash".as_bytes(), salt, &config).unwrap();
                if argon2::verify_encoded(m.password.as_str(), password.as_bytes()).unwrap() {
                    return create_new_session(hash.clone(), user.unwrap().unwrap().id, conn)
                        .await
                        .map(|_| LoginUserResponse {
                            access_token: hash.split("$").last().unwrap().to_owned(),
                        });
                } else {
                    return Err(ServiceError::InvalidEmailOrPassword);
                }
            }
        }
    }

    Err(ServiceError::DBQueryError(format!("{:?}", &user.err())))
}

async fn create_new_session(
    hash: String,
    user_id: i32,
    conn: &DatabaseConnection,
) -> Result<(), ServiceError> {
    let session = Session::find()
        .filter(session::Column::UserId.eq(user_id.clone()))
        .one(conn)
        .await;
    match session.as_ref().unwrap() {
        Some(_) => {
            Session::delete_by_id(session.unwrap().unwrap().id)
                .exec(conn)
                .await
                .ok();
        }
        None => {}
    }
    Session::insert(session::ActiveModel {
        token: sea_orm::ActiveValue::Set(hash.split("$").last().unwrap().to_owned()),
        user_id: sea_orm::ActiveValue::Set(user_id),
        ..Default::default()
    })
    .exec_with_returning(conn)
    .await
    .map(|_| ())
    .map_err(|e| ServiceError::DBQueryError(format!("{:?}", e)))
}

pub async fn valid_token(token: &str, db: &DatabaseConnection) -> bool {
    let session = Session::find()
        .filter(session::Column::Token.eq(token))
        .one(db)
        .await;
    if let Err(_) = session {
        return false;
    }
    println!("{:?}", &session.as_ref().unwrap());
    match &session.unwrap() {
        None => false,
        Some(_) => true,
    }
}
