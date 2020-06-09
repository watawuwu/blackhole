mod args;

use anyhow::Result;
use args::Args;
use blackhole_bin::server;
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
