use axum::{Extension, Json};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use diesel::{Connection, OptionalExtension, RunQueryDsl};

use crate::entity::reactions::{Reaction, ReactionInsert, ReactionUser, ReactionUserInsert};
use crate::entity::reactions::reactions::dsl::reactions as reactionsTable;
use crate::entity::reactions::reaction_users::dsl::reaction_users as reactionUsersTable;
use crate::entity::reactions::reaction_users::reaction_id as reactionUsersTableReactionId;
use crate::entity::reactions::reactions::message_id;
use crate::entity::reactions::reactions::reaction_id;
use crate::entity::user::User;
use crate::server::rest::{CompletePrivateMessage, error, IrisResponse, no_content, ok, ReactionAddRequest};
use crate::SharedState;
use http_body_util::BodyExt;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::entity::reactions::reaction_users::user_id;

pub async fn add_reaction(
    Path(message_reaction_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<()> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let request = Json::from_bytes(request.into_body().collect().await.unwrap().to_bytes().as_ref());
    if request.is_err() {
        return error(StatusCode::BAD_REQUEST, "Invalid reaction");
    }
    let request: ReactionAddRequest = request.unwrap().0;

    let database = &mut state.write().await.database;
    let transaction_result = database.transaction::<_, diesel::result::Error, _>(|connection|  {
        let reaction = reactionsTable
            .filter(message_id.eq(message_reaction_id))
            .get_result::<Reaction>(connection)
            .optional()?;

        let message_reaction_id: Option<i32> = match reaction {
            Some(reaction) => Some(reaction.reaction_id),
            None => {
                let new_reaction = ReactionInsert {
                    message_id: message_reaction_id,
                    emoji: request.reaction_type
                };
                let query = diesel::insert_into(reactionsTable)
                    .values(&new_reaction)
                    .returning(reaction_id)
                    .get_result::<i32>(connection)?;

                Some(query)
            }
        };
        if message_reaction_id.is_none() {
            return Err(diesel::result::Error::NotFound);
        }

        let reaction_user = ReactionUserInsert {
            reaction_id: message_reaction_id.unwrap(),
            user_id: user.id,
        };

        let user_query = diesel::insert_into(reactionUsersTable)
            .values(&reaction_user)
            .execute(connection)?;
        Ok(())
    });

    if transaction_result.is_err() {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to add reaction");
    }

    no_content()
}

pub async fn remove_reaction(
    Path(message_reaction_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<()> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let database = &mut state.write().await.database;
    let transaction_result = database.transaction::<_, diesel::result::Error, _>(|connection|  {
        let reaction = reactionsTable
            .filter(message_id.eq(message_reaction_id))
            .get_result::<Reaction>(connection)?;

        let reaction_user = reactionUsersTable
            .filter(reactionUsersTableReactionId.eq(reaction.reaction_id))
            .filter(user_id.eq(user.id))
            .get_result::<ReactionUser>(connection)
            .optional()?;

        if reaction_user.is_none() {
            return Err(diesel::result::Error::NotFound);
        }

        let user_query = diesel::delete(reactionUsersTable)
            .filter(reactionUsersTableReactionId.eq(reaction.reaction_id))
            .filter(user_id.eq(user.id))
            .execute(connection)?;
        Ok(())
    });

    if transaction_result.is_err() {
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to remove reaction");
    }

    no_content()
}