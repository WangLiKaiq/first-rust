use crate::user::UserId;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: UserId,
    pub exp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClaimsToken(String);

impl ClaimsToken {
    pub fn from(claims: Claims, secret: SecretString) -> Self {
        let raw = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.expose_secret().as_bytes()),
        )
        .unwrap();
        Self(raw)
    }

    pub fn to_claims(
        self: &Self,
        secret: SecretString,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            self.0.as_str(),
            &DecodingKey::from_secret(secret.expose_secret().as_bytes()),
            &Validation::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use lib::{rand::rand_string, time::clock::now_jst};
    use secrecy::SecretString;

    use crate::user::{
        UserId,
        token::{Claims, ClaimsToken},
    };

    #[tokio::test]
    async fn test_verify() {
        let subject = UserId::rand();
        let exp = now_jst().unix_timestamp() as u64;
        let claims = Claims {
            sub: subject.clone(),
            exp: exp,
        };
        let secret = SecretString::from(rand_string(100));
        let token = ClaimsToken::from(claims, secret.clone());
        let actual_subject = token.to_claims(secret);
        assert!(actual_subject.is_ok());
        let token_data = actual_subject.unwrap();
        assert_eq!(token_data.claims.exp, exp);
        assert_eq!(token_data.claims.sub, subject);
        println!("{:?}", token_data.header)
    }
}
