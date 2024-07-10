use std::collections::BTreeMap;
use axum::{Extension, Json};
use axum::http::{HeaderMap, StatusCode};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use jwt::VerifyWithKey;
use crate::entity::user::User;
use crate::entity::user::users::dsl::users;
use crate::server::rest;
use crate::server::rest::{IrisResponse, UserSelfResponse};
use crate::SharedState;

pub async fn get_self(
    headers: HeaderMap,
    Extension(state): Extension<SharedState>
) -> IrisResponse<UserSelfResponse> {
    let token_header = headers.get("Authorization");
    println!("Arrived");
    if token_header.is_none() {
        println!("Debug 1");
        return rest::error(StatusCode::UNAUTHORIZED, "No token provided")
    }
    let token = token_header.unwrap().to_str().unwrap().split_whitespace().last();
    if token.is_none() {
        println!("Debug 2");
        return rest::error(StatusCode::UNAUTHORIZED, "No token provided")
    }

    let token = token.unwrap();
    let mut state = state.write().unwrap();
    let claims = token.verify_with_key(&state.jwt_key);
    if claims.is_err() {
        println!("Debug 3");
        return rest::error(StatusCode::UNAUTHORIZED, "Invalid token")
    }

    let claims: BTreeMap<String, i64> = claims.unwrap();
    let user_id = claims.get("id");
    if user_id.is_none() {
        return rest::error(StatusCode::UNAUTHORIZED, "Invalid token")
    }

    let user_id = user_id.unwrap();
    let user = users
        .find(user_id)
        .select(User::as_select())
        .first::<User>(&mut state.database);

    if let Ok(user) = user {
        rest::ok(UserSelfResponse::from(user))
    } else {
        println!("Debug 6");
        rest::error(StatusCode::UNAUTHORIZED, "Invalid token")
    }
}