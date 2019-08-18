mod args;
mod error;
mod fs;

use futures::{Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};

use exitcode;
use log::*;
use std::env;

use crate::args::Args;
use crate::error::Result;

use chrono::prelude::*;
use std::process::exit;

use ansi_term::Style;
use ansi_term::{ANSIGenericString, Colour};

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn gulp(max_chars: u32, req: Request<Body>) -> BoxFut {
    let headers = req.headers().clone();
    let method = req.method().clone();
    let uri = req.uri().clone();
    let content_type = headers
        .get("content-type")
        .map(|h| h.to_str().unwrap_or(""))
        .map(String::from)
        .unwrap_or_default();

    let reversed = req.into_body().concat2().map(move |chunk| {
        let body_value = if is_logging(&content_type) {
            String::from_utf8_lossy(&chunk.to_vec())
                .to_string()
                .chars()
                .take(max_chars as usize)
                .collect::<String>()
        } else {
            String::new()
        };
        let local = Local::now();

        println!(
            r#"{{{method_label}: {method_value}, {path_label}: {path_value}, {query_label}: {query_value}, {headers_label}: {headers_value}, {body_label}: {body_value}, {ts_label}: {ts_value}}}"#,
            method_label = label("method"),
            method_value = value(method.as_str()),
            path_label = label("path"),
            path_value = value(uri.path()),
            query_label = label("query"),
            query_value = value(uri.query().unwrap_or("")),
            headers_label = label("headers"),
            headers_value = Style::new().fg(Colour::Green).paint(format!(r#"{:?}"#, headers)),
            body_label = label("body"),
            body_value = value(body_value),
            ts_label = label("ts"),
            ts_value = value(local.to_rfc3339_opts(SecondsFormat::Secs, true)),
        );

        Response::new(Body::empty())
    });

    Box::new(reversed)
}

fn is_logging(mime: &str) -> bool {
    mime.contains("text/")
        || mime.contains("application/json")
        || mime.contains("application/xml")
        || mime.contains("application/x-www-form-urlencoded")
}

fn label<'a, S: Into<String>>(input: S) -> ANSIGenericString<'a, str> {
    Style::new()
        .bold()
        .fg(Colour::Blue)
        .paint(format!(r#""{}""#, input.into()))
}

fn value<'a, S: Into<String>>(input: S) -> ANSIGenericString<'a, str> {
    Style::new()
        .fg(Colour::Green)
        .paint(format!(r#""{}""#, input.into()))
}

fn serve(row_args: Vec<String>) -> Result<()> {
    let args = Args::new(&row_args)?;

    if let Some(level) = args.log_level() {
        env::set_var("RUST_LOG", level);
    }
    pretty_env_logger::init();

    debug!("args: {:?}", args);

    let addr = args.socket_addr();
    let max_chars = args.max_chars;
    let service = move || service_fn(move |req: Request<Body>| gulp(max_chars, req));

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    debug!("Listening on http://{}", addr);
    hyper::rt::run(server);
    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let code = match serve(args) {
        Ok(_) => exitcode::OK,
        Err(err) => {
            eprintln!("{}", err);
            exitcode::USAGE
        }
    };
    exit(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use futures::sync::oneshot;
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    use tokio::runtime::Runtime;

    fn ok(server: &Serve, msg: &[u8]) -> Result<()> {
        let mut req = connect(&server.addr)?;
        req.write_all(msg)?;
        let mut response = String::new();
        req.read_to_string(&mut response)?;
        assert!(response.contains("HTTP/1.1 200 OK"));
        Ok(())
    }

    #[test]
    fn anything_ok() -> Result<()> {
        let server = serve();

        let get = b"\
        GET / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-get\r\n\
        Accept: */*\r\n\
        \r\n\
        ";
        ok(&server, &get.to_vec())?;

        let nested_path = b"\
        GET /n1/n2/n3 HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-nested_path\r\n\
        Accept: */*\r\n\
        \r\n\
        ";
        ok(&server, &nested_path.to_vec())?;

        let query = b"\
        GET /query?param=1 HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-query\r\n\
        Accept: */*\r\n\
        \r\n\
        ";
        ok(&server, &query.to_vec())?;

        let head = b"\
        HEAD / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-head\r\n\
        Accept: */*\r\n\
        \r\n\
        ";
        ok(&server, &head.to_vec())?;

        let option = b"\
        OPTION / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-option\r\n\
        Accept: */*\r\n\
        \r\n\
        ";
        ok(&server, &option.to_vec())?;

        let put = b"\
        PUT / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-put\r\n\
        Accept: */*\r\n\
        \r\n\
        ";
        ok(&server, &put.to_vec())?;

        let delete = b"\
        DELETE / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-delete\r\n\
        Accept: */*\r\n\
        \r\n\
        ";
        ok(&server, &delete.to_vec())?;

        let post = b"\
        POST / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-content_type\r\n\
        Content-Length: 7\r\n\
        \r\n\
        aaa=bbb\
        ";
        ok(&server, &post.to_vec())?;

        let chunked = b"\
        POST / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-chunked\r\n\
        Transfer-Encoding: chunked\r\n\
        \r\n\
        2\r\n\
        rt\r\n\
        0\r\n\
        \r\n\
        ";
        ok(&server, &chunked.to_vec())?;

        let content_type = b"\
        POST / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-content_type\r\n\
        Content-Type: application/x-www-form-urlencoded\r\n\
        Content-Length: 7\r\n\
        \r\n\
        aaa=bbb\
        ";
        ok(&server, &content_type.to_vec())?;

        let ignore_logging = b"\
        POST / HTTP/1.1\r\n\
        Host: example.domain\r\n\
        User-Agent: test-ingnore_loggging\r\n\
        Content-type: application/octet-stream\r\n\
        Content-Length: 7\r\n\
        \r\n\
        aaa=bbb\
        ";
        ok(&server, &ignore_logging.to_vec())?;

        Ok(())
    }

    struct Serve {
        pub addr: SocketAddr,
        shutdown_signal: Option<oneshot::Sender<()>>,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Drop for Serve {
        fn drop(&mut self) {
            drop(self.shutdown_signal.take());
            self.thread.take().unwrap().join().unwrap();
        }
    }

    fn serve() -> Serve {
        let (addr_tx, addr_rx) = mpsc::channel();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let addr = ([127, 0, 0, 1], 0).into();
        let thread_name = format!(
            "test-server-{}",
            thread::current()
                .name()
                .unwrap_or("<unknown test case name>")
        );

        let thread = thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                let service = move || service_fn(move |req: Request<Body>| gulp(1024, req));
                let server = Server::bind(&addr)
                    .http1_only(true)
                    .http1_keepalive(false)
                    .http1_pipeline_flush(true)
                    .serve(service);

                addr_tx.send(server.local_addr()).expect("server addr tx");

                let fut = server.with_graceful_shutdown(shutdown_rx);

                let mut rt = Runtime::new().expect("rt new");

                rt.block_on(fut).unwrap();
            })
            .expect("thread spawn");

        let addr = addr_rx.recv().expect("server addr rx");

        Serve {
            addr,
            shutdown_signal: Some(shutdown_tx),
            thread: Some(thread),
        }
    }

    fn connect(addr: &SocketAddr) -> Result<TcpStream> {
        let req = TcpStream::connect(addr)?;
        req.set_read_timeout(Some(Duration::from_secs(1)))?;
        req.set_write_timeout(Some(Duration::from_secs(1)))?;
        Ok(req)
    }
}
