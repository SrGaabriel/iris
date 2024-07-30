
use diesel::allow_tables_to_appear_in_same_query;
use serde::Serialize;

pub mod users;
pub mod messages;
pub mod reactions;
pub mod channels;

use crate::schema::users::users as users_table;
use crate::schema::channels::channels as channels_table;
use crate::schema::channels::channel_members as channel_members_table;
use crate::schema::messages::messages as messages_table;
use crate::schema::reactions::reactions as reactions_table;
use crate::schema::reactions::reaction_users as reaction_users_table;

allow_tables_to_appear_in_same_query!(
    users_table,
    channels_table,
    channel_members_table,
    messages_table,
    reactions_table,
    reaction_users_table
);