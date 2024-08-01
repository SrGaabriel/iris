use axum::http::StatusCode;
use axum::Json;
use axum_extra::either::Either;
use serde::{Deserialize, Serialize};
use crate::schema::reactions::ReactionSummary;
pub use crate::schema::users::User;

pub mod auth;
pub mod contacts;
pub mod user;
pub mod middlewares;
pub mod messages;
pub(crate) mod reactions;
pub(crate) mod search;

pub type IrisResponse<T> = (StatusCode, Either<Json<T>, Json<IrisError>>);

#[derive(Serialize)]
pub struct IrisError {
    pub status: u16,
    pub message: String
}

pub fn ok<T: Serialize>(data: T) -> IrisResponse<T> {
    (StatusCode::OK, Either::E1(Json(data)))
}

pub fn no_content() -> IrisResponse<()> {
    (StatusCode::NO_CONTENT, Either::E1(Json(())))
}

pub fn error<T: Serialize>(status: StatusCode, message: &str) -> IrisResponse<T> {
    (status, Either::E2(Json(IrisError {
        status: status.as_u16(),
        message: String::from(message)
    })))
}

#[derive(Serialize)]
pub struct UserAuthResponse {
    pub user: UserSelfResponse,
    pub token: String
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
            id: user.user_id,
            name: user.name,
            username: user.username,
            email: user.email
        }
    }
}

#[derive(Serialize)]
pub struct PrimordialMessage {
    pub id: i64,
    pub content: String,
    pub receipt: i16
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StandardUser {
    pub id: i64,
    pub name: String,
    pub username: String
}

#[derive(Serialize)]
pub struct ContactResponse {
    pub user_id: i64,
    pub channel_id: i64,
    pub name: String,
    pub username: String,
    pub last_message: Option<PrimordialMessage>,
    pub unread_count: i64
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct MessageObject {
    pub id: i64,
    pub content: String,
    pub user_id: i64,
    pub channel_id: i64,
    pub receipt: i16,
    pub edited: bool,
    pub author: StandardUser,
    pub reply_to: Option<i64>,
    pub reactions: Vec<ReactionSummary>
}

#[derive(Deserialize)]
pub struct ReactionAddRequest {
    pub reaction_id: Option<i32>,
    pub reaction_type: String,
}

#[derive(Serialize)]
pub struct ReactionAddResponse {
    pub reaction_id: i32,
    pub reaction_count: i32
}

#[derive(Serialize)]
pub struct GeneralSearchResponse {
    pub users: Vec<StandardUser>
}

#[derive(Serialize)]
pub struct PrivateChannel {
    pub channel_id: i64
}