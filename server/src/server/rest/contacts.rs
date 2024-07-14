use axum::body::Body;
use axum::Extension;
use axum::http::Request;
use diesel::prelude::*;
use diesel::row::NamedRow;
use diesel::{debug_query, RunQueryDsl, sql_query};
use diesel::sql_types::BigInt;

use crate::entity::message::ContactWithLastMessage;
use crate::entity::user::User;
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
        WITH recent_messages AS (
            SELECT u.*, m.id AS message_id, m.content, m.reception_status,
                   (SELECT COUNT(*)
                    FROM messages
                    WHERE (user_id = u.id AND context = $1)
                      AND reception_status = 0) AS reception_status_count
            FROM users u
            LEFT JOIN LATERAL (
                SELECT id, content, reception_status
                FROM messages
                WHERE (user_id = u.id AND context = $1)
                   OR (context = u.id AND user_id = $1)
                ORDER BY id DESC
                LIMIT 1
            ) m ON true
            WHERE u.id != $1
        )
        SELECT *
        FROM recent_messages
        ORDER BY COALESCE(message_id, -1) DESC;
    ").bind::<BigInt, _>(user.id);
    let results = query
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
                    content: contact.content.unwrap(),
                    receipt: contact.reception_status.unwrap(),
                }),
                None => None
            },
            unread_count: contact.reception_status_count
        }
    }).collect();

    ok(result)
}