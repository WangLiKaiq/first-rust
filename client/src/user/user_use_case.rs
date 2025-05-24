use std::sync::Arc;

use anyhow::{Result, anyhow};
use sea_orm::{DatabaseConnection, TransactionTrait};
use secrecy::SecretString;

use crate::server::AppState;

use super::{
    UserId,
    authentication::{HashedPassword, PasswordSalt, RawPassword},
    fetch_by_username,
    token::{ClaimsToken, create_token},
    users_repo::{SaveUser, get_stored_credentials, save_user},
};
#[derive(Debug)]
pub enum CreateUserError {
    UserAlreadyExisting,
}
pub async fn create_new_user(
    state: &AppState,
    username: String,
    password: RawPassword,
    email: SecretString,
) -> Option<CreateUserError> {
    let hashed_password = HashedPassword::hash(&password, PasswordSalt::rand()).unwrap();
    let tx = state.db.begin().await.unwrap();
    let user = fetch_by_username(&tx, &username).await;
    if let Ok(_) = user {
        return Some(CreateUserError::UserAlreadyExisting);
    }

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
    None
}

pub async fn user_login(
    conn: Arc<DatabaseConnection>,
    username: String,
    password: RawPassword,
) -> Result<ClaimsToken> {
    let txn = conn.begin().await?;

    let credentials = get_stored_credentials(&username, &txn).await?;

    match credentials {
        Some((user_id, hashed_password)) => match password.verify(&hashed_password) {
            Ok(true) => {
                let token = create_token(user_id.clone())?;
                Ok(token)
            }
            _ => Err(anyhow!("Invalid username or password")),
        },
        None => Err(anyhow!("Invalid username or password")),
    }
}
