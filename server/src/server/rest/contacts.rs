use axum::body::Body;
use axum::Extension;
use axum::http::Request;
use diesel::associations::HasTable;
use diesel::row::NamedRow;
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, Table, NullableExpressionMethods, sql_query, debug_query};
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use diesel::sql_types::BigInt;
use crate::entity::message::messages::{context, context_type, user_id};
use crate::entity::message::messages::dsl::messages;
use crate::entity::user::User;
use crate::entity::user::users::dsl::users;
use crate::entity::user::users as usersTable;
use crate::entity::message::{ContactWithLastMessage, messages as messagesTable};
use crate::server::rest::{ContactResponse, IrisResponse, ok, PrimordialMessage};
use crate::SharedState;

// For the time being, we will return all registered users as contacts
pub async fn get_contacts(
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<Vec<ContactResponse>> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let conn = &mut state.write().await.database;

    let query = sql_query("
        SELECT u.*, m.id AS message_id, m.content
        FROM users u
        LEFT JOIN LATERAL (
            SELECT id, content
            FROM messages
            WHERE (user_id = u.id AND context = $1)
               OR (context = u.id AND user_id = $1)
            ORDER BY id DESC
            LIMIT 1
        ) m ON true
        WHERE u.id != $1
        ORDER BY COALESCE(m.id, -1) DESC;
    ");
    let results = query
        .bind::<BigInt, _>(user.id)
        .load::<ContactWithLastMessage>(conn)
        .expect("Failed to load contacts");

    let result = results.into_iter().map(|contact| {
        ContactResponse {
            id: contact.id,
            name: contact.name,
            username: contact.username.clone(),
            last_message: match contact.message_id {
                Some(_) => Some(PrimordialMessage {
                    id: contact.message_id.unwrap(),
                    content: contact.content.unwrap()
                }),
                None => None
            }
        }
    }).collect();

    ok(result)
}