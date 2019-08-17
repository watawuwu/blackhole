use crate::error::Result;
use std::net::{Ipv4Addr, SocketAddr};
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
pub struct Args {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Listen port
    #[structopt(short, long, default_value = "3000")]
    pub port: u16,

    /// Listen address
    #[structopt(short, long, default_value = "0.0.0.0")]
    pub addr: Ipv4Addr,

    /// Max display body chars
    #[structopt(short, long, default_value = "1024")]
    pub max_chars: u32,
}

impl Args {
    pub fn new(raw_args: &[String]) -> Result<Args> {
        let mut app = Args::clap();
        let mut buf: Vec<u8> = Vec::new();
        app.write_long_help(&mut buf)?;

        let clap = app.get_matches_from_safe(raw_args)?;
        let args = Args::from_clap(&clap);
        Ok(args)
    }

    pub fn socket_addr(&self) -> SocketAddr {
        (self.addr, self.port).into()
    }

    pub fn log_level(&self) -> Option<String> {
        let level = match self.verbose {
            1 => Some("error"),
            2 => Some("warn"),
            3 => Some("info"),
            4 => Some("debug"),
            5 => Some("trace"),
            _ => None,
        };
        level.map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_level_ok(test_args: Vec<&str>, expect: &str) {
        let raw_args = test_args.into_iter().map(String::from).collect::<Vec<_>>();
        let args = Args::new(&raw_args).unwrap();
        assert_eq!(args.log_level(), Some(String::from(expect)));
    }

    #[test]
    fn log_level() {
        let raw_args = vec!["blackhole", "-v"];
        log_level_ok(raw_args, "error");

        let raw_args = vec!["blackhole", "-vv"];
        log_level_ok(raw_args, "warn");

        let raw_args = vec!["blackhole", "-vvv"];
        log_level_ok(raw_args, "info");

        let raw_args = vec!["blackhole", "-vvvv"];
        log_level_ok(raw_args, "debug");

        let raw_args = vec!["blackhole", "-vvvvv"];
        log_level_ok(raw_args, "trace");
    }
}
