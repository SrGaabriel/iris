use axum::body::Body;
use axum::Extension;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use diesel::prelude::*;
use diesel::row::NamedRow;
use diesel::{debug_query, RunQueryDsl, sql_query};
use diesel::sql_types::BigInt;
use crate::schema::channels::{Channel, PrivateChannelQuery};
use crate::schema::messages::ContactWithChannel;
use crate::schema::users::User;
use crate::server::rest::{ContactResponse, error, IrisResponse, ok, PrimordialMessage, PrivateChannel};
use crate::SharedState;

// For the time being, we will return all registered users as contacts
pub async fn get_contacts(
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<Vec<ContactResponse>> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let conn = &mut state.write().await.database;

    let query = sql_query("
WITH last_messages AS (
    SELECT
        m.channel_id,
        m.message_id,
        m.content,
        m.reception_status
    FROM
        messages m
    WHERE
        m.message_id = (
            SELECT MAX(m2.message_id)
            FROM messages m2
            WHERE m2.channel_id = m.channel_id
        )
),
unread_reception_count AS (
    SELECT
        m.channel_id,
        COUNT(*) AS unread_reception_count
    FROM
        messages m
    WHERE
        m.reception_status = 0 AND m.user_id != $1
    GROUP BY
        m.channel_id
)

SELECT DISTINCT
    u.user_id,
    cm1.channel_id,
    u.name,
    u.username,
    lm.message_id,
    lm.content,
    lm.reception_status,
    COALESCE(urc.unread_reception_count, 0) AS unread_reception_count
FROM
    users u
JOIN
    channel_members cm1 ON u.user_id = cm1.user_id
JOIN
    channels c ON cm1.channel_id = c.channel_id
JOIN
    channel_members cm2 ON cm1.channel_id = cm2.channel_id
LEFT JOIN
    last_messages lm ON cm1.channel_id = lm.channel_id
LEFT JOIN
    unread_reception_count urc ON cm1.channel_id = urc.channel_id
WHERE
    cm2.user_id = $1
    AND c.channel_type = 0
    AND u.user_id != $1;
    ").bind::<BigInt, _>(user.user_id);
    let results = query
        .load::<ContactWithChannel>(conn)
        .expect("Failed to load contacts");

    let result = results.into_iter().map(|contact| {
        ContactResponse {
            user_id: contact.user_id,
            channel_id: contact.channel_id,
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
            unread_count: 0 // TODO: use the actual value
        }
    }).collect();

    ok(result)
}

pub async fn get_contact(
    Path(contact_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<ContactResponse> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let conn = &mut state.write().await.database;

    let query = sql_query("
        WITH recent_messages AS (
            SELECT u.*, m.id AS message_id, m.content, m.reception_status,
                   (SELECT COUNT(*)
                    FROM messages
                    WHERE (user_id = $2 AND context = $1)
                      AND reception_status = 0) AS reception_status_count
            FROM users u
            LEFT JOIN LATERAL (
                SELECT id, content, reception_status
                FROM messages
                WHERE (user_id = $2 AND context = $1)
                   OR (context = $2 AND user_id = $1)
                ORDER BY id DESC
                LIMIT 1
            ) m ON true
            where u.id = $1
        )
        SELECT *
        FROM recent_messages
        ORDER BY COALESCE(message_id, -1) DESC;
    ").bind::<BigInt, _>(contact_id).bind::<BigInt, _>(user.user_id);
    let results = query
        .load::<ContactWithChannel>(conn)
        .expect("Failed to load contact");

    let contact = results.into_iter().next().expect("Contact not found");

    let result = ContactResponse {
        user_id: contact.user_id,
        channel_id: contact.channel_id,
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
        unread_count: contact.unread_reception_count
    };

    ok(result)
}

// This will simply return the channel between the two users if it exists, or create it if it doesn't
pub async fn chat(
    Path(contact_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<PrivateChannel> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let state = &mut state.write().await;

    let snowflake_id = {
        &state.snowflake_issuer.generate()
    };

    let query = sql_query(r#"
    WITH existing_channel AS (
        SELECT cm1.channel_id
        FROM channel_members cm1
        JOIN channel_members cm2 ON cm1.channel_id = cm2.channel_id
        JOIN channels c ON cm1.channel_id = c.channel_id
        WHERE cm1.user_id = $1
        AND cm2.user_id = $2
        AND c.channel_type = 0
    ), inserted_channel AS (
        INSERT INTO channels (channel_id, channel_type)
        SELECT $3, 0
        WHERE NOT EXISTS (SELECT 1 FROM existing_channel)
        RETURNING channel_id
    ), new_channel_member1 AS (
        INSERT INTO channel_members (channel_id, user_id)
        SELECT channel_id, $1
        FROM inserted_channel
        RETURNING channel_id
    ), new_channel_member2 AS (
        INSERT INTO channel_members (channel_id, user_id)
        SELECT channel_id, $2
        FROM inserted_channel
        RETURNING channel_id
    )
    SELECT channel_id FROM existing_channel
    UNION ALL
    SELECT channel_id FROM inserted_channel
    UNION ALL
    SELECT channel_id FROM new_channel_member1
    UNION ALL
    SELECT channel_id FROM new_channel_member2
    LIMIT 1;
    "#).bind::<BigInt, _>(user.user_id).bind::<BigInt, _>(contact_id).bind::<BigInt, _>(snowflake_id.value() as i64);
    let channel_id_result = query.load::<PrivateChannelQuery>(&mut state.database);

    if channel_id_result.is_err() {
        channel_id_result.unwrap();
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get channel");
    }

    let channel = channel_id_result.unwrap().into_iter().next().expect("Channel not found");
    ok(PrivateChannel {
        channel_id: channel.channel_id
    })
}