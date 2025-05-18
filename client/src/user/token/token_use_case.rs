use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use uuid::Uuid;

use super::Claims;

const SECRET: &[u8] = b"your-secret-key";

pub fn create_token(user_id: Uuid) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp() as u64;

    let claims = Claims {
        subject: user_id,
        expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .unwrap()
}

pub fn verify_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::user::token::create_token;

    #[tokio::test]
    async fn test_create_token() {
        let token = create_token(Uuid::new_v4());

        println!("The generated token is: {}", token);
    }
}
