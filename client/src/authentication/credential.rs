use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use lib::rand::rand_b64;
use secrecy::{ExposeSecret, SecretString};

static SALT_LENGTH: usize = 64;
#[derive(Debug)]
pub struct Credentials {
    pub username: String,
    pub password: RawPassword,
}

#[derive(Debug, Clone)]
pub struct RawPassword(pub SecretString);

#[derive(Debug, Clone)]
pub struct HashedPassword(pub SecretString);
pub struct PasswordSalt(pub SaltString);

impl RawPassword {
    /// Verifies this raw password against a stored hash.
    #[tracing::instrument(name = "Validate credentials", skip(self, expected_hash))]
    pub fn verify(&self, expected_hash: &HashedPassword) -> Result<bool> {
        let parsed_hash = PasswordHash::new(expected_hash.0.expose_secret())
            .context("Invalid stored password hash")?;

        let result =
            Argon2::default().verify_password(self.0.expose_secret().as_bytes(), &parsed_hash);

        Ok(result.is_ok())
    }
}

impl HashedPassword {
    /// Hashes a raw password using Argon2 and the provided salt.
    pub fn hash(raw: &RawPassword, salt: PasswordSalt) -> Result<Self> {
        let hashed = Argon2::default()
            .hash_password(raw.0.expose_secret().as_bytes(), &salt.0)?
            .to_string()
            .into_boxed_str();

        Ok(Self(SecretString::new(hashed)))
    }
}

impl PasswordSalt {
    pub fn rand() -> Self {
        Self(SaltString::from_b64(rand_b64(SALT_LENGTH).as_str()).unwrap())
    }
}
