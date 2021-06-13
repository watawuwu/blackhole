use chrono::prelude::*;

use anyhow::*;
use colored_json::{ColorMode, ColoredFormatter, CompactFormatter};
use serde::Serialize;
use std::collections::HashMap;

use log::*;
use serde_json::Value;
use std::str::FromStr;
use tide::Request;

#[derive(Debug, Serialize)]
pub struct AccessLog {
    path: String,
    query: HashMap<String, String>,
    addr: String,
    #[serde(flatten)]
    body: Option<Body>,
    headers: HashMap<String, String>,
    method: String,
    ts: DateTime<Local>,
}

#[derive(Debug, Serialize)]
enum Body {
    #[serde(rename = "body")]
    JsonValue(Value),
    #[serde(rename = "body")]
    Text(String),
}

impl AccessLog {
    pub async fn from<State: Send + Sync + 'static>(mut req: Request<State>) -> tide::Result<Self> {
        let method = req.method().to_string();
        let path = req.url().path().to_string();
        let query = req.query()?;
        let addr = req.local_addr().map(String::from).unwrap_or_default();
        let content_type = req
            .content_type()
            .map(|x| x.to_string())
            .unwrap_or_default();

        let body = if Self::is_record_body(&content_type) {
            let body = req.body_string().await?;
            let body = match serde_json::Value::from_str(&body) {
                Ok(v) => Body::JsonValue(v),
                Err(_) => Body::Text(body),
            };
            Some(body)
        } else {
            None
        };
        let headers = req
            .iter()
            .map(|(k, v)| (String::from(k.as_str()), String::from(v.as_str())))
            .collect();

        let ts = Local::now();

        Ok(AccessLog {
            path,
            query,
            addr,
            body,
            headers,
            method,
            ts,
        })
    }

    pub fn log(&self) -> Result<()> {
        let value = serde_json::to_value(self)?;
        let formatter = ColoredFormatter::new(CompactFormatter {});
        let json = formatter.to_colored_json(&value, ColorMode::On)?;
        info!("{}", json);
        Ok(())
    }

    fn is_record_body(content_type: &str) -> bool {
        content_type.contains("text/")
            || content_type.contains("application/json")
            || content_type.contains("application/xml")
            || content_type.contains("application/x-www-form-urlencoded")
    }
}
