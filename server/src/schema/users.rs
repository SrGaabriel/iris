use diesel::{Identifiable, Insertable, Queryable, Selectable};

#[derive(Debug, Identifiable, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = users)]
#[diesel(primary_key(user_id))]
pub struct User {
    pub user_id: i64,
    pub name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

diesel::table! {
    users (user_id) {
        user_id -> BigInt,
        name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar
    }
}