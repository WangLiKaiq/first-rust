use actix_web::{Responder, web};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::server::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Dummy)]
pub struct RegisterRequest {
    name: String,
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
pub async fn register(
    _state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    tracing::info!("state: , request: {:?}", req);

    web::Json(RegisterResponse {
        business_error: Some("Dummy biz error".to_string()),
    })
}
