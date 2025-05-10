use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use secrecy::{ExposeSecret, SecretString};

pub struct Credentials {
    pub username: String,
    pub password: RawPassword,
}

pub struct RawPassword(SecretString);
pub struct HashedPassword(SecretString);

impl RawPassword {
    #[tracing::instrument(name = "Get confirmed subscribers", skip(self, expected_password_hash))]
    pub fn validate_credentials(
        &self,
        expected_password_hash: HashedPassword,
    ) -> Result<bool, anyhow::Error> {
        let expected_password_hash = PasswordHash::new(&expected_password_hash.0.expose_secret())
            .context("Failed to parse user password.")?;

        let result = Argon2::default()
            .verify_password(self.0.expose_secret().as_bytes(), &expected_password_hash);
        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl HashedPassword {
    pub fn hash(raw_password: RawPassword, salt: &SaltString) -> Result<Self, anyhow::Error> {
        let hash = Argon2::default()
            .hash_password(raw_password.0.expose_secret().as_bytes(), salt)?
            .to_string()
            .into_boxed_str();

        Ok(HashedPassword(SecretString::new(hash)))
    }
}
