use diesel::{Identifiable, Queryable, Selectable, table};
use crate::schema::users::users;

#[derive(Queryable, Identifiable)]
#[table_name = "channels"]
#[primary_key(channel_id)]
pub struct Channel {
    pub channel_id: i64,
    pub channel_type: i32,
}

#[derive(Queryable, Identifiable, Selectable)]
#[table_name = "channel_members"]
#[primary_key(channel_id, user_id)]
pub struct ChannelMember {
    pub channel_id: i64,
    pub user_id: i64
}

table! {
    channels (channel_id) {
        channel_id -> BigInt,
        channel_type -> Integer,
    }
}

table! {
    channel_members (channel_id, user_id) {
        channel_id -> BigInt,
        user_id -> BigInt,
        joined_at -> Timestamp,
    }
}

diesel::joinable!(channel_members -> channels (channel_id));
diesel::joinable!(channel_members -> users (user_id));