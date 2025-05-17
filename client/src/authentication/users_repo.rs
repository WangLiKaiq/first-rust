use anyhow::{Context, Result};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

use crate::entities::users;

use super::HashedPassword;

pub struct UserRepository {}
#[tracing::instrument(name = "Get stored credentials", skip(db))]
pub async fn get_stored_credentials(
    username: &str,
    db: &DatabaseConnection,
) -> Result<Option<(Uuid, HashedPassword)>> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .context("Failed to fetch user from DB")?;

    Ok(user.and_then(|u| {
        Uuid::from_slice(&u.id).ok().map(|uuid| {
            let hash = HashedPassword(SecretString::new(u.password_hash.into_boxed_str()));
            (uuid, hash)
        })
    }))
}

#[derive(Clone)]
pub struct SaveUser {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<HashedPassword>,
    pub email: Option<SecretString>,
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

    match existing {
        Some(model) => {
            // Update existing
            let mut active_model = model.into_active_model();
            if let Some(username) = user.username {
                active_model.username = Set(username);
            }
            if let Some(password) = user.password {
                active_model.password_hash = Set(password.0.expose_secret().to_owned());
            }
            if let Some(email) = user.email {
                active_model.email = Set(email.expose_secret().to_owned());
            }

            active_model.update(db).await?;
        }
        None => {
            let uuid = user.id.unwrap_or(Uuid::new_v4());
            // Insert new
            let new = users::ActiveModel {
                id: Set(uuid.as_bytes().to_vec()),
                username: Set(user.username.unwrap()),
                password_hash: Set(user.password.unwrap().0.expose_secret().to_string()),
                email: Set(user.email.unwrap().expose_secret().to_string()),
                created_at: Set(Some(Utc::now())),
            };
            new.insert(db).await?;
        }
    }

    Ok(())
}
