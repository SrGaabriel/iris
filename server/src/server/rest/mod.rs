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

#[derive(Serialize)]
pub struct ContactResponse {
    pub user_id: i64,
    pub channel_id: i64,
    pub name: String,
    pub username: String,
    pub last_message: Option<PrimordialMessage>,
    pub unread_count: i64
}

#[derive(Serialize)]
pub struct PrivateMessage {
    pub id: i64,
    pub content: String,
    pub user_id: i64,
    pub context: i64,
    pub receipt: i16,
    pub edited: bool,
    pub reply_to: Option<i64>
}

#[derive(Serialize)]
pub struct CompletePrivateMessage {
    pub id: i64,
    pub content: String,
    pub user_id: i64,
    pub context: i64,
    pub receipt: i16,
    pub edited: bool,
    pub reply_to: Option<PrivateMessage>
}

impl CompletePrivateMessage {
    pub fn with_reply(message: &crate::schema::messages::Message, reply_message: Option<PrivateMessage>) -> Self {
        CompletePrivateMessage {
            id: message.message_id,
            content: String::from(&message.content),
            user_id: message.user_id,
            context: message.channel_id,
            receipt: message.reception_status,
            edited: message.edited,
            reply_to: reply_message
        }
    }
}

impl From<&crate::schema::messages::Message> for PrivateMessage {
    fn from(message: &crate::schema::messages::Message) -> Self {
        PrivateMessage {
            id: message.message_id,
            content: String::from(&message.content),
            user_id: message.user_id,
            context: message.channel_id,
            receipt: message.reception_status,
            edited: message.edited,
            reply_to: message.reply_to
        }
    }
}

#[derive(Serialize)]
pub struct IterablePrivateMessage {
    pub id: i64,
    pub content: String,
    pub user_id: i64,
    pub context: i64,
    pub receipt: i16,
    pub edited: bool,
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