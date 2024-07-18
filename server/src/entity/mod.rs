use serde::Serialize;
use crate::server::rest::UserSelfResponse;

pub mod user;
pub mod message;
pub mod reactions;

#[derive(Serialize)]
pub struct UserAuthResponse {
    pub user: UserSelfResponse,
    pub token: String
}