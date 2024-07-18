use axum::{Extension, Json};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use diesel::{Connection, OptionalExtension, RunQueryDsl};

use crate::entity::reactions::{Reaction, ReactionInsert, ReactionUser, ReactionUserInsert};
use crate::entity::reactions::reactions::dsl::reactions as reactionsTable;
use crate::entity::reactions::reaction_users::dsl::reaction_users as reactionUsersTable;
use crate::entity::reactions::reaction_users::reaction_id as reactionUsersTableReactionId;
use crate::entity::reactions::reactions::{emoji, message_id, reaction_count};
use crate::entity::reactions::reactions::reaction_id;
use crate::entity::user::User;
use crate::server::rest::{CompletePrivateMessage, error, IrisResponse, no_content, ok, ReactionAddRequest, ReactionAddResponse};
use crate::SharedState;
use http_body_util::BodyExt;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::entity::reactions::reaction_users::user_id;

pub async fn add_reaction(
    Path(message_reaction_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<ReactionAddResponse> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let request = Json::from_bytes(request.into_body().collect().await.unwrap().to_bytes().as_ref());
    if request.is_err() {
        return error(StatusCode::BAD_REQUEST, "Invalid reaction");
    }
    let request: ReactionAddRequest = request.unwrap().0;

    let database = &mut state.write().await.database;
    let transaction_result = database.transaction::<_, diesel::result::Error, _>(|connection|  {
        let reaction_details: Option<(i32, i32)> = match request.reaction_id {
            Some(message_reaction_id) => {
                let count = diesel::update(reactionsTable)
                    .filter(reaction_id.eq(message_reaction_id))
                    .set(reaction_count.eq(reaction_count + 1))
                    .returning(reaction_count)
                    .get_result::<i32>(connection)?;
                Some((message_reaction_id, count))
            },
            None => {
                let reaction = diesel::update(reactionsTable)
                    .filter(message_id.eq(message_reaction_id))
                    .filter(emoji.eq(request.reaction_type.clone()))
                    .set(reaction_count.eq(reaction_count + 1))
                    .returning((reaction_id, reaction_count))
                    .get_result::<(i32, i32)>(connection)
                    .optional()?;

                if let Some(tuple) = reaction {
                    println!("Already found a reaction, no need for new one");
                    Some(tuple)
                } else {
                    println!("Inserted");
                    let new_reaction = ReactionInsert {
                        message_id: message_reaction_id,
                        emoji: request.reaction_type
                    };
                    let query = diesel::insert_into(reactionsTable)
                        .values(&new_reaction)
                        .returning(reaction_id)
                        .get_result::<i32>(connection)?;

                    Some((query, 1))
                }
            }
        };
        if reaction_details.is_none() {
            return Err(diesel::result::Error::NotFound);
        }

        let (message_reaction_id, count) = reaction_details.unwrap();
        let reaction_user = ReactionUserInsert {
            reaction_id: message_reaction_id,
            user_id: user.id,
        };

        let user_query = diesel::insert_into(reactionUsersTable)
            .values(&reaction_user)
            .execute(connection)?;
        Ok((message_reaction_id, count))
    });

    if transaction_result.is_err() {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to add reaction");
    }
    let (message_reaction_id, count) = transaction_result.unwrap();

    ok(ReactionAddResponse {
        reaction_id: message_reaction_id,
        reaction_count: count
    })
}

pub async fn remove_reaction(
    Path((message_identifier, reaction_identifier)): Path<(i64, i32)>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<()> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let database = &mut state.write().await.database;
    let transaction_result = database.transaction::<_, diesel::result::Error, _>(|connection|  {
        // reduce one from reaction count
        let reaction = diesel::update(reactionsTable)
            .filter(reaction_id.eq(reaction_identifier))
            .set(reaction_count.eq(reaction_count - 1))
            .execute(connection)?;

        let user_query = diesel::delete(reactionUsersTable)
            .filter(reactionUsersTableReactionId.eq(reaction_identifier))
            .filter(user_id.eq(user.id))
            .execute(connection)?;
        Ok(())
    });

    if transaction_result.is_err() {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to remove reaction");
    }

    no_content()
}