use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use log::LevelFilter;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Parser, Debug)]
#[command(author, version, about, next_line_help = true, long_about = None)]
pub struct Args {
    /// Color mode off
    #[arg(
        short = 'c',
        long = "no-color",
        default_value = "false",
        action(clap::ArgAction::SetTrue)
    )]
    pub no_color: bool,

    /// Listen address
    #[arg(short = 'a', long = "address", default_value = "127.0.0.1")]
    address: String,

    /// Listen port
    #[arg(short = 'p', long = "port", env = "PORT", default_value = "8080", value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
}

impl Args {
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let ipv4: Ipv4Addr = self.address.parse()?;
        Ok((ipv4, self.port).into())
    }

    pub fn log_level_filter(&self) -> LevelFilter {
        self.verbose.log_level_filter()
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
