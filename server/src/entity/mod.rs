use serde::Serialize;
use crate::server::rest::UserSelfResponse;

pub mod user;
pub mod message;

#[derive(Serialize)]
pub struct UserAuthResponse {
    pub user: UserSelfResponse,
    pub token: String
}