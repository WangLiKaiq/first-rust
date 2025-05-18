use actix_web::{Scope, web};

use crate::user::{login, register};

pub fn user_router() -> Scope {
    web::scope("/user")
        .route("/register/v1", web::post().to(register))
        .route("/login/v1", web::post().to(login))
}
