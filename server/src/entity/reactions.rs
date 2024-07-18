use diesel::{allow_tables_to_appear_in_same_query, Identifiable, Insertable, Queryable, Selectable};
use crate::entity::message::messages;
use crate::entity::user::users;
use diesel::sql_types::{BigInt, Text, Varchar, Integer, Bool};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Queryable, Selectable, Insertable, Clone)]
#[diesel(belongs_to(Message))]
#[diesel(table_name = reactions)]
#[diesel(primary_key(reaction_id))]
pub struct Reaction {
    pub reaction_id: i32,
    pub message_id: i64,
    pub emoji: String,
    pub reaction_count: i32
}

diesel::table! {
    reactions (reaction_id) {
        reaction_id -> Integer,
        message_id -> BigInt,
        emoji -> Varchar,
        reaction_count -> Integer
    }
}

#[derive(Debug, Identifiable, Queryable, Selectable, Insertable, Clone)]
#[diesel(belongs_to(Reaction))]
#[diesel(belongs_to(User))]
#[diesel(table_name = reaction_users)]
#[diesel(primary_key(id))]
pub struct ReactionUser {
    pub id: i32,
    pub reaction_id: i32,
    pub user_id: i64
}


diesel::table! {
    reaction_users (id) {
        id -> Integer,
        reaction_id -> Integer,
        user_id -> BigInt
    }
}

diesel::joinable!(reactions -> messages (message_id));
diesel::joinable!(reaction_users -> reactions (reaction_id));
diesel::joinable!(reaction_users -> users (user_id));

allow_tables_to_appear_in_same_query!(
    messages,
    reactions,
    reaction_users,
);

#[derive(Insertable)]
#[diesel(table_name = reactions)]
pub struct ReactionInsert {
    pub message_id: i64,
    pub emoji: String,
}

#[derive(Insertable)]
#[diesel(table_name = reaction_users)]
pub struct ReactionUserInsert {
    pub reaction_id: i32,
    pub user_id: i64
}

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct ReactionSummary {
    #[sql_type = "Text"]
    pub emoji: String,
    #[sql_type = "Integer"]
    pub count: i32,
    #[sql_type = "Bool"]
    pub me: bool,
    #[sql_type = "Integer"]
    pub reaction_id: i32
}


