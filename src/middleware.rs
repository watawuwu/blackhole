use std::future::Future;
use std::pin::Pin;

use crate::access_log::AccessLog;
use log::*;
use std::fmt::Debug;

use tide::{Middleware, Next, Request, Response, StatusCode};

pub struct AccessLogMiddleware;

impl AccessLogMiddleware {
    pub fn new() -> Self {
        Self {}
    }

    async fn log<'a, State: Send + Sync + 'static>(
        &'a self,
        req: Request<State>,
        _next: Next<'a, State>,
    ) -> tide::Result {
        let access = AccessLog::from(req).await?;
        if let Err(err) = access.log() {
            error!("Failed to print access log. err: {}", err)
        }
        Ok(Response::new(StatusCode::Ok))
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for AccessLogMiddleware {
    fn handle<'a>(
        &'a self,
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
        Box::pin(async move { self.log(ctx, next).await })
    }
}

impl Debug for AccessLogMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessLogMiddleware").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::*;

    use std::sync::mpsc::{channel, Sender};

    use console::strip_ansi_codes;
    use serde_json::Value;
    use tide::http::{Method, Request, Response, Url};

    fn log_init(tx: Sender<String>) -> Result<()> {
        fern::Dispatch::new()
            .format(|out, message, _record| out.finish(format_args!("{}", message)))
            .level(log::LevelFilter::Info)
            .level_for("tide", log::LevelFilter::Warn)
            .chain(tx)
            .apply()?;
        Ok(())
    }

    // TODO no good test case
    //      log writes are in global scope
    #[async_std::test]
    async fn status_ok() -> Result<()> {
        let (tx, rx) = channel();
        log_init(tx)?;

        let mut app = tide::new();
        app.middleware(AccessLogMiddleware::new());

        let mut req = Request::new(Method::Get, Url::parse("http://localhost/foo?test=1")?);
        req.append_header("Content-Type", "text/plain");
        req.set_body("param1=1&param2=2");
        let _: Response = app.respond(req).await.unwrap();
        let color_log = rx.recv()?;
        let log = strip_ansi_codes(&color_log);

        let a: Value = serde_json::from_str(&log)?;
        assert_eq!(a["method"].as_str().unwrap(), "GET");
        assert_eq!(a["path"].as_str().unwrap(), "/foo");
        assert_eq!(a["query"]["test"].as_str().unwrap(), "1");
        assert_eq!(a["headers"]["content-type"].as_str().unwrap(), "text/plain");
        assert_eq!(a["body"].as_str().unwrap(), "param1=1&param2=2");

        let mut req = Request::new(Method::Post, Url::parse("http://localhost/bar")?);
        req.append_header("Content-Type", "application/json");
        req.set_body(r#"{"name": "watawuwu", "job": "engineer"}"#);

        let _: Response = app.respond(req).await.unwrap();
        let color_log = rx.recv()?;
        let log = strip_ansi_codes(&color_log);

        let a: Value = serde_json::from_str(&log)?;
        assert_eq!(a["method"].as_str().unwrap(), "POST");
        assert_eq!(a["path"].as_str().unwrap(), "/bar");
        assert_eq!(
            a["headers"]["content-type"].as_str().unwrap(),
            "application/json"
        );
        assert_eq!(a["body"]["name"].as_str().unwrap(), "watawuwu");
        assert_eq!(a["body"]["job"].as_str().unwrap(), "engineer");

        Ok(())
    }
}
