use actix_identity::Identity;
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use actix_web::middleware::Next;
use actix_web::{FromRequest, HttpMessage};
use uuid::Uuid;

pub async fn user_id_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let (r, mut pl) = req.into_parts();
    let identity = Identity::from_request(&r, &mut pl).await.map_err(|_| ErrorUnauthorized(""))?;
    let user_id =
        Uuid::parse_str(identity.id().unwrap().as_str()).map_err(|_| ErrorBadRequest(""))?;
    let req = ServiceRequest::from_parts(r, pl);
    req.extensions_mut().insert(user_id);
    next.call(req).await
}
