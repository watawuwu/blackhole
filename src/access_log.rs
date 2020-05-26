use ansi_term::Style;
use ansi_term::{ANSIGenericString, Colour};
use chrono::prelude::*;

use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;

pub struct AccessLog {
    path: String,
    query: HashMap<String, String>,
    addr: Option<SocketAddr>,
    body: String,
    headers: HashMap<String, String>,
    content_type: String,
    method: String,
    ts: DateTime<Local>,
}

impl AccessLog {
    pub fn new(
        method: String,
        path: String,
        query: HashMap<String, String>,
        addr: Option<SocketAddr>,
        body: String,
        headers: HashMap<String, String>,
    ) -> AccessLog {
        let content_type = headers
            .get("content-type")
            .map(String::from)
            .unwrap_or_default();

        let ts = Local::now();

        AccessLog {
            path,
            query,
            addr,
            headers,
            body,
            content_type,
            method,
            ts,
        }
    }

    pub fn println(&self, max_chars: u32) {
        let body = if self.is_logging() {
            let buf = self.body.as_str();
            let v: Value = serde_json::from_str(buf).unwrap_or_else(|_| json!(buf));
            v.to_string()
                .chars()
                .take(max_chars as usize)
                .collect::<String>()
        } else {
            String::from(r#""""#)
        };
        println!(
            r#"{{{method_label}: {method_value}, {path_label}: {path_value}, {query_label}: {query_value}, {addr_label}: {addr_value}, {headers_label}: {headers_value}, {body_label}: {body_value}, {ts_label}: {ts_value}}}"#,
            method_label = label("method"),
            method_value = value(self.method.as_str()),
            path_label = label("path"),
            path_value = value(self.path.as_str()),
            query_label = label("query"),
            query_value = Style::new()
                .fg(Colour::Green)
                .paint(format!(r#"{:?}"#, self.query)),
            addr_label = label("addr"),
            addr_value = value(
                self.addr
                    .map(|s| s.to_string())
                    .unwrap_or_default()
                    .as_str()
            ),
            headers_label = label("headers"),
            headers_value = Style::new()
                .fg(Colour::Green)
                .paint(format!(r#"{:?}"#, self.headers)),
            body_label = label("body"),
            body_value = Style::new().fg(Colour::Green).paint(body),
            ts_label = label("ts"),
            ts_value = value(self.rfc3339().as_str()),
        );
    }

    fn rfc3339(&self) -> String {
        self.ts.to_rfc3339_opts(SecondsFormat::Secs, true)
    }

    fn is_logging(&self) -> bool {
        self.content_type.contains("text/")
            || self.content_type.contains("application/json")
            || self.content_type.contains("application/xml")
            || self
                .content_type
                .contains("application/x-www-form-urlencoded")
    }
}

fn label(input: &str) -> ANSIGenericString<str> {
    Style::new()
        .bold()
        .fg(Colour::Blue)
        .paint(format!(r#""{}""#, input))
}

fn value(input: &str) -> ANSIGenericString<str> {
    let json: Value = json!(input);
    Style::new()
        .fg(Colour::Green)
        .paint(format!(r#""{}""#, json.as_str().unwrap_or_default()))
}
