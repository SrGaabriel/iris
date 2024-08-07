use diesel::{Identifiable, Insertable, Queryable, Selectable};
use crate::schema::messages::messages;
use crate::schema::users::users;
use diesel::sql_types::{Text, Integer, Bool, Serial};
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
        reaction_id -> Serial,
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
        id -> Serial,
        reaction_id -> Integer,
        user_id -> BigInt
    }
}

diesel::joinable!(reactions -> messages (message_id));
diesel::joinable!(reaction_users -> reactions (reaction_id));
diesel::joinable!(reaction_users -> users (user_id));

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

#[derive(Debug, Serialize, Deserialize, QueryableByName, Clone, PartialEq)]
pub struct ReactionSummary {
    #[diesel(sql_type = Text)]
    pub emoji: String,
    #[diesel(sql_type = Integer)]
    pub count: i32,
    #[diesel(sql_type = Bool)]
    pub me: bool,
    #[diesel(sql_type = Serial)]
    pub reaction_id: i32
}


