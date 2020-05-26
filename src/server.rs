use crate::args::Args;
use anyhow::*;
use std::env;

use crate::access_log::AccessLog;
use log::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::body::FullBody;

use warp::filters::path::FullPath;
use warp::http::{HeaderMap, Method};
use warp::{self, Buf, Filter};

fn blackhole(
    mac_chars: u32,
    method: Method,
    path: FullPath,
    query: HashMap<String, String>,
    addr: Option<SocketAddr>,
    body: FullBody,
    headers: HeaderMap,
) -> impl warp::Reply {
    let header_map = headers
        .iter()
        .map(|(h, v)| (h.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<HashMap<_, _>>();

    let access_log = Arc::new(AccessLog::new(
        method.to_string(),
        path.as_str().into(),
        query,
        addr,
        String::from_utf8_lossy(&body.bytes().to_vec()).to_string(),
        header_map,
    ));
    access_log.clone().println(mac_chars);
    warp::reply()
}
fn service(
    max_chars: u32,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Copy {
    let _a = warp::log::custom(|_info| println!("dddd"));

    warp::any()
        .and(warp::any().map(move || max_chars))
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::addr::remote())
        .and(warp::body::concat())
        .and(warp::header::headers_cloned())
        .map(blackhole)
}

pub fn serve(row_args: Vec<String>) -> Result<()> {
    let args = Args::new(&row_args)?;

    if let Some(level) = args.log_level() {
        env::set_var("RUST_LOG", level);
    }
    pretty_env_logger::init();

    let socket = args.socket_addr();
    let max_chars = args.max_chars;

    debug!("start server {}", socket);

    let service = service(max_chars);
    warp::serve(service).run(socket);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches() {
        let filter = service(1024);

        let res = warp::test::request().path("/").reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request().path("/1/2").reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request().path("/?aaa=bbb").reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request()
            .method("POST")
            .body("aaa=bbb")
            .path("/post")
            .reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request().method("DELETE").reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request().method("PUT").reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request().method("OPTIONS").reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request().method("HEAD").reply(&filter);
        assert_eq!(res.status(), 200);

        let res = warp::test::request().method("TRANCE").reply(&filter);
        assert_eq!(res.status(), 200);

        let headers = vec![
            ("Content-Type", "application/json"),
            ("Content-Type", "application/xml"),
            ("Content-Type", "application/x-www-form-urlencoded"),
            ("Content-Type", "application/octet-stream"),
            ("Content-Type", "application/pdf"),
            ("Content-Type", "image/gif"),
            ("Content-Type", "image/jpg"),
            ("Content-Type", "image/png"),
        ];
        headers.into_iter().for_each(|(h, v)| {
            let res = warp::test::request().header(h, v).reply(&filter);
            assert_eq!(res.status(), 200);
        });
    }
}
