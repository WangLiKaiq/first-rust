use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHasher};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

use crate::entities::users;

use super::Credentials;

#[tracing::instrument(name = "Get stored credentials", skip(db))]
pub async fn get_stored_credentials(
    username: &str,
    db: &DatabaseConnection,
) -> Result<Option<(Uuid, SecretString)>> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .context("Failed to fetch user from DB")?;

    Ok(user.and_then(|u| {
        Uuid::from_slice(&u.id).ok().map(|uuid| {
            let hash = SecretString::new(u.password_hash.into_boxed_str());
            (uuid, hash)
        })
    }))
}

#[derive(Clone)]
pub struct SaveUser {
    id: Option<Uuid>,
    username: Option<String>,
    password: Option<SecretString>,
    email: Option<SecretString>,
}
pub async fn save_user(db: &DatabaseConnection, user: SaveUser) -> Result<()> {
    // Attempt to find existing user by ID if provided
    let existing = if let Some(id) = user.id {
        users::Entity::find_by_id(id).one(db).await?
    } else if let Some(ref username) = user.username {
        users::Entity::find()
            .filter(users::Column::Username.eq(username.clone()))
            .one(db)
            .await?
    } else {
        None
    };

    // Convert Option<T> to Set<T> for ActiveModel
    fn to_set<T>(value: Option<T>) -> sea_orm::Set<T> {
        match value {
            Some(v) => Set::Value(v),
            None => Set::Unset,
        }
    }

    match existing {
        Some(model) => {
            // Update existing
            let mut active_model = model.into_active_model();
            if let Some(username) = user.username {
                active_model.username = Set(username);
            }
            if let Some(password) = user.password {
                active_model.password_hash = Set(password.expose_secret().clone());
            }
            if let Some(email) = user.email {
                active_model.email = Set(email.expose_secret().clone());
            }

            active_model.update(db).await?;
        }
        None => {
            // Insert new
            let new = users::ActiveModel {
                id: user.id.map(Set).unwrap_or(Set(Uuid::new_v4())),
                username: to_set(user.username),
                password_hash: to_set(user.password.map(|p| p.expose_secret().clone())),
                email: to_set(user.email.map(|e| e.expose_secret().clone())),
                ..Default::default()
            };
            new.insert(db).await?;
        }
    }

    Ok(())
}
