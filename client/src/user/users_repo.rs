use crate::entities::users;
use anyhow::{Context, Result};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, Set,
};
use secrecy::{ExposeSecret, SecretString};

use super::{UserId, authentication::HashedPassword};

pub struct UserRepository {}
#[tracing::instrument(name = "Get stored credentials", skip(db))]
pub async fn get_stored_credentials<C>(
    username: &str,
    db: &C,
) -> Result<Option<(UserId, HashedPassword)>>
where
    C: ConnectionTrait,
{
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .context("Failed to fetch user from DB")?;

    Ok(user.and_then(|u| {
        let user_id: UserId =
            UserId::from_bytes(&u.id).unwrap_or_else(|_| panic!("Invalid userId: {:?}", u.id));
        let hash = HashedPassword(SecretString::new(u.password_hash.into_boxed_str()));
        Some((user_id, hash))
    }))
}

#[derive(Clone)]
pub struct SaveUser {
    pub id: Option<UserId>,
    pub username: Option<String>,
    pub password: Option<HashedPassword>,
    pub email: Option<SecretString>,
}
pub async fn save_user(db: &DatabaseConnection, user: SaveUser) -> Result<()> {
    // Attempt to find existing user by ID if provided
    let existing = if let Some(id) = user.id.clone() {
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
            let uuid = user.id.unwrap_or(UserId::rand());
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
