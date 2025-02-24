use std::future::{ready, Ready};

use crate::services::auth::is_token_expired;
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::{self, header::HeaderValue},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

pub struct JWTSession;

impl<S, B> Transform<S, ServiceRequest> for JWTSession
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTSessionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTSessionMiddleware { service }))
    }
}
pub struct JWTSessionMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JWTSessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        // Change this to see the change in outcome in the browser.
        // Usually this boolean would be acquired from a password check or other auth verification.
        let token = request
            .headers()
            .get("Authorization")
            .ok_or("")
            .unwrap_or(&HeaderValue::from_static("Bearer "))
            .to_str()
            .unwrap_or("")
            .to_string()
            .replace("Bearer ", "");
        let is_exp = match is_token_expired(&token) {
            Ok(is_exp) => is_exp,
            Err(_err) => true,
        };

        // if the token is expired redirect to login
        if is_exp {
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Found()
                .insert_header((http::header::LOCATION, "/auth/login"))
                .finish()
                // constructed responses map to "right" body
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(request);

        Box::pin(async move {
            // forwarded responses map to "left" body
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}

