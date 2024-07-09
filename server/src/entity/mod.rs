use serde::Serialize;
use crate::server::rest::UserSelfResponse;

pub mod user;

#[derive(Serialize)]
pub struct UserAuthResponse {
    pub user: UserSelfResponse,
    pub token: String
}