use axum::http::StatusCode;
use axum::Json;
use axum_extra::either::Either;
use serde::Serialize;
use crate::entity::user::User;

pub mod auth;
pub mod user;

pub type IrisResponse<T> = (StatusCode, Either<Json<T>, Json<IrisError>>);

#[derive(Serialize)]
pub struct IrisError {
    pub status: u16,
    pub message: String
}

pub fn ok<T: Serialize>(data: T) -> IrisResponse<T> {
    (StatusCode::OK, Either::E1(Json(data)))
}

pub fn error<T: Serialize>(status: StatusCode, message: &str) -> IrisResponse<T> {
    (status, Either::E2(Json(IrisError {
        status: status.as_u16(),
        message: String::from(message)
    })))
}

#[derive(Serialize)]
pub struct UserSelfResponse {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String
}

impl From<User> for UserSelfResponse {
    fn from(user: User) -> Self {
        UserSelfResponse {
            id: user.id,
            name: user.name,
            username: user.username,
            email: user.email
        }
    }
}