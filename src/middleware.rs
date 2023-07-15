use crate::access_log::AccessLog;
use actix_web::http::StatusCode;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;

pub struct StructuredLogging;

impl<S: 'static, B> Transform<S, ServiceRequest> for StructuredLogging
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = StructuredLoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(StructuredLoggingMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct StructuredLoggingMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for StructuredLoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let accesslog = AccessLog::from(&mut req).await;
            accesslog
                .log()
                .unwrap_or_else(|_| log::warn!("parse error"));

            // re-insert body back into request to be used by handlers
            // req.set_payload(bytes_to_payload(body));
            let mut res = svc.call(req).await?;

            let status = res.response_mut().status_mut();
            *status = StatusCode::OK;
            Ok(res)
        })
    }
}
