use actix_web::{Scope, web};

use crate::handler::user::register;

pub fn user_router() -> Scope {
    web::scope("/user").route("/register/v1", web::post().to(register))
}
