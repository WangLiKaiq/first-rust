use secrecy::SecretString;

use crate::server::AppState;

use super::{
    authentication::{HashedPassword, PasswordSalt, RawPassword},
    users_repo::{SaveUser, get_stored_credentials, save_user},
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

pub async fn user_login(state: &AppState, username: String, password: RawPassword) -> bool {
    let credentials = get_stored_credentials(username.as_str(), &state.db)
        .await
        .unwrap();

    if let Some((_, hashed_password)) = credentials {
        match password.verify(&hashed_password) {
            Ok(result) => result,
            Err(_) => false,
        }
    } else {
        false
    }
}
