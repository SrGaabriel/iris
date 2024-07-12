use chrono::NaiveDateTime;
use diesel::{allow_tables_to_appear_in_same_query, Associations, Identifiable, Insertable, Queryable, Selectable};
use crate::entity::user::users;
use crate::User;

pub type ContextType = i32;

#[derive(Queryable, Identifiable, Associations, Selectable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub context: i64,
    pub context_type: ContextType,
}

diesel::table! {
    use diesel::sql_types::*;

    messages (id) {
        id -> BigInt,
        user_id -> BigInt,
        content -> Text,
        context -> BigInt,
        context_type -> Integer,
    }
}

diesel::joinable!(messages -> users (user_id));

allow_tables_to_appear_in_same_query!(
    users,
    messages,
);