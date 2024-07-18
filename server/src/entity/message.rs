use diesel::pg::Pg;
use diesel::sql_types::{Nullable, Text, BigInt, SmallInt};
use diesel::{allow_tables_to_appear_in_same_query, Associations, Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use crate::entity::user::users;
use crate::User;
use diesel::sql_types::Bool;

pub type ContextType = i16;

#[derive(Queryable, Identifiable, Associations, Selectable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub context: i64,
    pub context_type: ContextType,
    pub reception_status: i16,
    pub edited: bool,
    pub reply_to: Option<i64>
}

diesel::table! {
    use diesel::sql_types::*;

    messages (id) {
        id -> BigInt,
        user_id -> BigInt,
        content -> Text,
        context -> BigInt,
        context_type -> SmallInt,
        reception_status -> SmallInt,
        edited -> Bool,
        reply_to -> Nullable<BigInt>
    }
}

diesel::joinable!(messages -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    messages,
);

#[derive(QueryableByName, Queryable, Debug)]
pub struct ContactWithLastMessage {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Text"]
    pub username: String,
    #[sql_type = "Nullable<BigInt>"]
    pub message_id: Option<i64>,
    #[sql_type = "Nullable<Text>"]
    pub content: Option<String>,
    #[sql_type = "Nullable<SmallInt>"]
    pub reception_status: Option<i16>,
    #[sql_type = "BigInt"]
    pub reception_status_count: i64
}

#[derive(QueryableByName, Queryable, Debug)]
pub struct CompleteMessage {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "BigInt"]
    pub user_id: i64,
    #[sql_type = "Text"]
    pub content: String,
    #[sql_type = "BigInt"]
    pub context: i64,
    #[sql_type = "SmallInt"]
    pub reception_status: i16,
    #[sql_type = "Bool"]
    pub edited: bool,
    #[sql_type = "Nullable<BigInt>"]
    pub reply_to: Option<i64>,
    #[sql_type = "Text"]
    pub reactions: String
}