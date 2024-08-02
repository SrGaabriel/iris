use std::collections::BTreeMap;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum::response::IntoResponse;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use jwt::VerifyWithKey;

use crate::schema::users::User;
use crate::schema::users::users::dsl::users;
use crate::schema::users::users::user_id as table_user_id;
use crate::server::rest::error;
use crate::SharedState;

pub async fn authorize(mut req: Request, next: Next) -> Response {
    let headers = req.headers().clone();
    let auth = headers.get("Authorization").or(headers.get("Sec-Websocket-Protocol"));
    if auth.is_none() {
        return error::<String>(StatusCode::UNAUTHORIZED, "No authorization header provided").into_response();
    }

    let auth = auth.unwrap().to_str();
    if auth.is_err() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid authorization header").into_response();
    }

    let parts: Vec<&str> = auth.unwrap().split_whitespace().collect();
    let token = parts.last();
    if token.is_none() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    let extensions = req.extensions_mut();
    let user = {
        let mut state = extensions.get_mut::<SharedState>().unwrap().write().await;
        let claims: Result<BTreeMap<String, i64>, jwt::error::Error> = token.unwrap().verify_with_key(&state.jwt_key);
        if claims.is_err() {
            return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }
        let claims = claims.unwrap();

        let user_id: Option<&i64> = claims.get("id");
        if user_id.is_none() {
            return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
        }

        users
            .filter(table_user_id.eq(*user_id.unwrap()))
            .select(User::as_select())
            .first::<User>(&mut state.database)
    };
    if user.is_err() {
        return error::<String>(StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    extensions.insert(user.unwrap());
    next.run(req).await
}