// use actix_web_lab::middleware::Next;
// use actix_web::body::MessageBody;
// use actix_web::dev::{ ServiceRequest, ServiceResponse };
// use crate::session_state::TypedSession;
// use crate::utils::e500;
// use actix_web::{FromRequest, HttpResponse};
// use actix_web::error::InternalError;

// pub async fn reject_anonymous_users(
//     mut req: ServiceRequest,
//     next: Next<impl MessageBody>
// ) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
//     let session = ({
//         let (http_request, payload) = req.parts_mut();
//         TypedSession::from_request(http_request, payload).await
//     })?;
//     match session.get_user_id().map_err(e500)? {
//         Some(_) => next.call(req).await,

//         None => {
//             let response =  HttpResponse::SeeOther()
//             .insert_header((actix_web::http::header::LOCATION, "/admin/dashboard"))
//             .finish();
//             let e = anyhow::anyhow!("The user has not logged in");
//             Err(InternalError::from_response(e, response).into())
//         }
//     }
// }
