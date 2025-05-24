use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{HttpMessage, HttpResponse};

use crate::constant::AUTHORIZATION;
use crate::user::token::{Claims, get_claims_from_header};

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    match extract_and_attach_claims(&mut req) {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("The authorization is failed due to: {e:?}");
            return Ok(
                req.into_response(HttpResponse::Unauthorized().body("Authorization failed."))
            );
        }
    }

    let res = next.call(req).await?;

    let (req, res_body) = res.into_parts();
    req.extensions_mut().remove::<Claims>();
    Ok(ServiceResponse::new(req, res_body.map_into_boxed_body()))
}

fn extract_and_attach_claims(req: &mut ServiceRequest) -> Result<(), anyhow::Error> {
    let header = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| anyhow::anyhow!("Missing authorization header."))?;

    let header_str = header.to_str()?;

    let claims = get_claims_from_header(header_str)?;
    req.extensions_mut().insert(claims);

    Ok(())
}
