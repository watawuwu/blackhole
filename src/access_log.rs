use anyhow::Result;

use std::collections::HashMap;

use actix_web::body::to_bytes;
use actix_web::dev::ServiceRequest;
use actix_web::{web, HttpMessage};
use log::*;
use serde::Serialize;
use serde_json::Value;
use time::serde::rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
pub struct AccessLog {
    #[serde(with = "rfc3339")]
    timestamp: OffsetDateTime,
    host: String,
    method: String,
    path: String,
    scheme: String,
    headers: HashMap<String, String>,
    query: String,
    req: Req,
}

#[derive(Debug, Serialize)]
struct Req {
    size: usize,
    #[serde(flatten)]
    body: Option<Body>,
}

#[derive(Debug, Serialize)]
enum Body {
    #[serde(rename = "body")]
    JsonValue(Value),
    #[serde(rename = "body")]
    Text(String),
}

impl AccessLog {
    pub async fn from(req: &mut ServiceRequest) -> Self {
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let query = req.query_string().to_string();
        let host = req.connection_info().host().to_string();
        let scheme = req.connection_info().scheme().to_string();
        let content_type = req.content_type().to_string();

        let buf = req.extract::<web::Bytes>().await.unwrap();

        let body = if Self::is_record_body(&content_type) {
            let body = match serde_json::from_slice::<Value>(&buf) {
                Ok(v) => Body::JsonValue(v),
                Err(_) => Body::Text(
                    std::str::from_utf8(buf.as_ref())
                        .unwrap_or_default()
                        .to_string(),
                ),
            };
            Some(body)
        } else {
            None
        };

        let size = match to_bytes(buf).await {
            Ok(buf) => buf.len(),
            Err(err) => {
                error!("err {:?}", err);
                0usize
            }
        };

        let headers = req
            .headers()
            .iter()
            .filter(|(k, _)| k.as_str() != "host")
            .map(|(k, v)| {
                (
                    String::from(k.as_str()),
                    String::from(v.to_str().unwrap_or_default()),
                )
            })
            .collect();

        let timestamp = OffsetDateTime::now_utc();

        AccessLog {
            timestamp,
            host,
            method,
            path,
            scheme,
            headers,
            query,
            req: Req { size, body },
        }
    }

    pub fn log(&self) -> Result<()> {
        let value = serde_json::to_value(self)?;
        info!("{}", value.to_string());
        Ok(())
    }

    fn is_record_body(content_type: &str) -> bool {
        content_type.contains("text/")
            || content_type.contains("application/json")
            || content_type.contains("application/x-www-form-urlencoded")
    }
}
