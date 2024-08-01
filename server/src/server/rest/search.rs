use axum::body::Body;
use axum::debug_handler;
use axum::extract::{Extension, Query, Request};
use axum::http::StatusCode;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};
use serde::Deserialize;
use crate::schema::users::User;
use crate::schema::users::users::dsl::users;
use crate::schema::users::users::user_id as table_user_id;
use crate::schema::users::users::name as table_users_name;
use crate::schema::users::users::username as table_users_username;
use crate::server::rest::{error, GeneralSearchResponse, IrisResponse, ok, StandardUser};
use crate::SharedState;

#[derive(Deserialize)]
pub struct Params {
    pub term: String
}

#[debug_handler]
pub async fn search(
    Extension(state): Extension<SharedState>,
    Query(params): Query<Params>,
    request: Request<Body>
) -> IrisResponse<GeneralSearchResponse> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let term = params.term.trim();
    if term.is_empty() {
        return error(StatusCode::BAD_REQUEST, "Empty search term");
    }

    let state = &mut state.write().await;
    let user_results = users
        .filter(
            table_user_id.ne(user.user_id)
                .and(
            table_users_name.like(format!("%{}%", term))
                .or(table_users_username.like(format!("%{}%", term))))
        )
        .select(User::as_select())
        .load::<User>(&mut state.database);

    if user_results.is_err() {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to search users");
    }

    let user_objects = user_results.unwrap().iter().map(|user| StandardUser {
        id: user.user_id,
        name: user.name.clone(),
        username: user.username.clone()
    }).collect();

    ok(GeneralSearchResponse {
        users: user_objects
    })
}