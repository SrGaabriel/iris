use diesel::{Insertable, Queryable, Selectable};

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

diesel::table! {
    users (id) {
        id -> BigInt,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        name -> Varchar
    }
}