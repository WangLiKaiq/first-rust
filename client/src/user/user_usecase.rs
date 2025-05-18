use secrecy::SecretString;

use crate::server::AppState;

use super::{
    authentication::{HashedPassword, PasswordSalt, RawPassword},
    users_repo::{SaveUser, save_user},
};

pub async fn create_new_user(
    state: &AppState,
    username: String,
    password: RawPassword,
    email: SecretString,
) {
    let hashed_password = HashedPassword::hash(&password, PasswordSalt::rand()).unwrap();
    save_user(
        &state.db,
        SaveUser {
            username: Some(username),
            password: Some(hashed_password),
            id: Some(uuid::Uuid::new_v4()),
            email: Some(email),
        },
    )
    .await
    .unwrap();
}
