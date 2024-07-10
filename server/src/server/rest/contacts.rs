use axum::body::Body;
use axum::Extension;
use axum::http::Request;
use diesel::associations::HasTable;
use diesel::row::NamedRow;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};

use crate::entity::user::User;
use crate::entity::user::users::dsl::users;
use crate::entity::user::users::id;
use crate::server::rest::{ContactResponse, IrisResponse, ok};
use crate::SharedState;

// For the time being, we will return all registered users as contacts
pub async fn get_contacts(
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<Vec<ContactResponse>> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let conn = &mut state.write().unwrap().database;
    let contacts = users::table()
        .filter(id.ne(user.id))
        .load::<User>(conn)
        .expect("Failed to load users");
    ok(contacts.iter().map(|u| ContactResponse::from(u)).collect())
}