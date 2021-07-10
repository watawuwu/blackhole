mod args;

use anyhow::Result;
use args::Args;
use async_std::prelude::*;
use blackhole_bin::server;
use signal_hook::consts::signal::*;
use signal_hook_async_std::Signals;
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

    let stop = async {
        let signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
        let mut signals = signals.fuse();
        let _ = signals.next().await;
        println!("Termination signal received, stopping server...");
        Ok(())
    };

    let app = async move {
        server::serve(args.socket_addr()?).await?;
        Ok::<_, anyhow::Error>(())
    };

    app.race(stop).await?;
    Ok(())
}
