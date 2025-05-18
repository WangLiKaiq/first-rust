mod user_controller;
mod user_id;
mod user_use_case;
mod users_repo;
use authentication::HashedPassword;
mod email;
pub use email::*;
pub use user_controller::*;
pub mod token;
pub use users_repo::*;
pub mod authentication;
pub use user_id::*;

use crate::entities::users;

pub struct User {
    user_id: UserId,
    username: String,
    password_hash: HashedPassword,
    email: Email,
}

impl TryFrom<users::Model> for User {
    type Error = anyhow::Error;

    fn try_from(u: users::Model) -> Result<Self, Self::Error> {
        Ok(User {
            user_id: UserId::from_bytes(&u.id)?,
            username: u.username,
            password_hash: u.password_hash.into(),
            email: Email::new(u.email)?,
        })
    }
}
