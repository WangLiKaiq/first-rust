use super::{Claims, ClaimsToken};
use crate::{
    constant::{TOKEN_DURATION_SECONDS, TOKEN_SECRET},
    user::UserId,
};
use anyhow::{Result, anyhow};
use lib::time::clock::now_jst;
use time::Duration;

pub fn create_token(user_id: UserId) -> Result<ClaimsToken> {
    let expiration = now_jst()
        .checked_add(Duration::seconds(TOKEN_DURATION_SECONDS))
        .ok_or_else(|| anyhow!("Failed to calculate the expiration time."))?
        .unix_timestamp() as u64;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    Ok(ClaimsToken::from(claims, TOKEN_SECRET.clone()))
}

pub fn get_claims_from_token(token: ClaimsToken) -> Result<Claims> {
    let token_data = token.to_claims(TOKEN_SECRET.clone())?;
    Ok(token_data.claims)
}
