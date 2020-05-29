use anyhow::Result;
use log::LevelFilter;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(structopt::StructOpt, paw_structopt::StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Args {
    #[structopt(flatten)]
    address: clap_flags::Address,
    #[structopt(flatten)]
    logger: clap_flags::Log,
    #[structopt(flatten)]
    port: clap_flags::Port,
}

impl Args {
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let ipv4: Ipv4Addr = self.address.address.parse()?;
        Ok((ipv4, self.port.port).into())
    }

    pub fn log_level_filter(&self) -> LevelFilter {
        self.logger
            .log_level()
            .map(|x| x.to_level_filter())
            .unwrap_or_else(|| LevelFilter::Info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use structopt::StructOpt;

    fn log_level_ok(test_args: Vec<&str>, expect: LevelFilter) -> Result<()> {
        let raw_args = test_args.into_iter().map(String::from).collect::<Vec<_>>();

        let app = Args::clap();
        let clap = app.get_matches_from_safe(raw_args)?;
        let args = Args::from_clap(&clap);
        assert_eq!(args.log_level_filter(), expect);
        Ok(())
    }

    #[test]
    fn log_level() -> Result<()> {
        let raw_args = vec!["blackhole"];
        log_level_ok(raw_args, LevelFilter::Info)?;

        let raw_args = vec!["blackhole", "-v"];
        log_level_ok(raw_args, LevelFilter::Debug)?;

        let raw_args = vec!["blackhole", "-vv"];
        log_level_ok(raw_args, LevelFilter::Trace)?;

        let raw_args = vec!["blackhole", "-vvv"];
        log_level_ok(raw_args, LevelFilter::Trace)?;

        Ok(())
    }
}
