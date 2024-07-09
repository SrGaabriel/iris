use std::collections::BTreeMap;

use argon2::{PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{Extension, Json};
use axum::http::StatusCode;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use jwt::SignWithKey;
use serde::Deserialize;
use rand::random;
use crate::entity::user::User;
use crate::entity::user::users::dsl::users;
use crate::entity::user::users::username;
use crate::entity::UserAuthResponse;
use crate::server::rest;
use crate::server::rest::{IrisResponse, ok, UserSelfResponse};
use crate::SharedState;

pub async fn login(
    Extension(state): Extension<SharedState>,
    Json(request): Json<LoginRequest>,
) -> IrisResponse<UserAuthResponse> {
    let state = &mut state.write().unwrap();
    let a = users
        .filter(username.eq(&request.identifier))
        .select(User::as_select())
        .first::<User>(&mut state.database);

    if let Ok(user) = a {
        let password_hash = PasswordHash::new(&*user.password).expect("Failed to create password hash");
        if state.argon.verify_password(request.password.as_bytes(), &password_hash).is_ok() {
            let mut claims = BTreeMap::new();
            claims.insert("id", user.id);
            let signed = claims.sign_with_key(&state.jwt_key).expect("Failed to sign JWT");

            return ok(UserAuthResponse {
                user: UserSelfResponse::from(user),
                token: signed
            })
        }
    }
    rest::error(StatusCode::UNAUTHORIZED, "Invalid credentials")
}

pub async fn register(
    Extension(state): Extension<SharedState>,
    Json(request): Json<RegisterRequest>,
) -> IrisResponse<UserAuthResponse> {
    let state = &mut state.write().unwrap();
    println!("Request password: {}", request.password);
    let hashed_password = state.argon.hash_password(request.password.as_bytes(), &state.argon_salt).expect("Failed to hash password");
    println!("Hashed password: {}", hashed_password);

    let new_user = User {
        id: random(),
        name: request.name.clone(),
        username: request.username.clone(),
        password: hashed_password.to_string(),
        email: request.email.clone()
    };

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(&mut state.database)
        .expect("Failed to insert user");

    let mut claims = BTreeMap::new();
    claims.insert("id", user.id);
    let signed = claims.sign_with_key(&state.jwt_key).expect("Failed to sign JWT");

    ok(UserAuthResponse {
        user: UserSelfResponse::from(user),
        token: signed
    })
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub name: String,
    pub username: String,
    pub password: String,
    pub email: String
}