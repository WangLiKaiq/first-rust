use std::{any, sync::Arc};

use anyhow::{Result, anyhow};
use sea_orm::{DatabaseConnection, TransactionTrait};
use secrecy::SecretString;

use crate::server::AppState;

use super::{
    UserId,
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
            id: Some(UserId::rand()),
            email: Some(email),
        },
    )
    .await
    .unwrap();
}

pub async fn user_login(
    conn: Arc<DatabaseConnection>,
    username: String,
    password: RawPassword,
) -> Result<UserId> {
    let txn = conn.begin().await?;

    let credentials = get_stored_credentials(&username, &txn).await?;

    match credentials {
        Some((user_id, hashed_password)) => match password.verify(&hashed_password) {
            Ok(true) => Ok(user_id),
            _ => Err(anyhow!("Invalid username or password")),
        },
        None => Err(anyhow!("Invalid username or password")),
    }
}
