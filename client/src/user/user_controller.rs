use actix_web::{HttpResponse, Responder, web};
use fake::Dummy;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{server::AppState, user::user_use_case::create_new_user};

use super::{authentication::RawPassword, user_use_case::user_login};

#[derive(Debug, Clone, Deserialize, ToSchema, Dummy)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Dummy)]
pub struct RegisterResponse {
    business_error: Option<String>,
}
#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/api/user/register",
    responses(
        (status = 200, description = "OK", body = [RegisterResponse]),
    )
)]
#[tracing::instrument(name = "Register a user", skip(state, req))]
pub async fn register(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    tracing::info!("state: , request: {:?}", req);
    let result = create_new_user(
        state.get_ref(),
        req.username.clone(),
        RawPassword(SecretString::from(req.password.clone())),
        SecretString::from(req.email.clone()),
    )
    .await;
    match result {
        None => web::Json(RegisterResponse {
            business_error: None,
        }),
        Some(_) => web::Json(RegisterResponse {
            business_error: Some("Failed to created a new user.".to_string()),
        }),
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum LoginResponse {
    Token {
        token: String,
        refresh_token: String,
        access_token: String,
        expire_in: u64,
    },
    Error {
        business_error_code: ErrorCode,
    },
}

#[derive(Debug, Clone, Serialize)]
pub enum ErrorCode {
    InvalidCredentials,
    AccountLocked,
    Unknown,
}

pub async fn login(state: web::Data<AppState>, req: web::Json<LoginRequest>) -> HttpResponse {
    let res = match user_login(
        state.db.clone(),
        req.username.clone(),
        RawPassword(SecretString::from(req.password.clone())),
    )
    .await
    {
        Ok(_) => LoginResponse::Token {
            token: "1234".to_string(),
            refresh_token: "1234".to_string(),
            access_token: "1234".to_string(),
            expire_in: 1,
        },
        Err(_) => LoginResponse::Error {
            business_error_code: ErrorCode::AccountLocked,
        },
    };
    HttpResponse::Ok().json(res)
}
