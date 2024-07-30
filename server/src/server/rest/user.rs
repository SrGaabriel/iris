use axum::body::Body;
use axum::http::Request;

use crate::schema::users::User;
use crate::server::rest;
use crate::server::rest::{IrisResponse, UserSelfResponse};

pub async fn get_self(
    request: Request<Body>
) -> IrisResponse<UserSelfResponse> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    rest::ok(UserSelfResponse::from(user))
}