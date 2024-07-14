use diesel::sql_types::{Nullable, Text, BigInt, SmallInt};
use diesel::{allow_tables_to_appear_in_same_query, Associations, Identifiable, Insertable, Queryable, QueryableByName, Selectable};
use crate::entity::user::users;
use crate::User;

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
    }
}

diesel::joinable!(messages -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    messages,
);

#[derive(QueryableByName, Debug)]
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