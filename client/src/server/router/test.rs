use actix_web::HttpResponse;

pub async fn test() -> HttpResponse {
    HttpResponse::Ok().finish()
}
