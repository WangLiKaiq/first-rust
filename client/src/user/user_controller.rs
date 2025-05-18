use actix_web::{Responder, web};
use fake::Dummy;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{server::AppState, user::user_usecase::create_new_user};

use super::authentication::RawPassword;

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
    create_new_user(
        state.get_ref(),
        req.username.clone(),
        RawPassword(SecretString::from(req.password.clone())),
        SecretString::from(req.email.clone()),
    )
    .await;
    web::Json(RegisterResponse {
        business_error: Some("Successfully created a new.".to_string()),
    })
}
