mod access_log;
mod args;
mod middleware;
mod server;

use anyhow::Result;
use args::Args;
use std::io;

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> Result<()> {
    let out = io::stdout();
    fern::Dispatch::new()
        .level(args.log_level_filter())
        .level_for("tide", log::LevelFilter::Warn)
        .chain(out)
        .apply()?;

    server::serve(args.socket_addr()?).await?;
    Ok(())
}
