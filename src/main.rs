use anyhow::Result;
use clap::Parser;

use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;

use args::Args;
use blackhole_bin::logger;
use blackhole_bin::server;
use futures_util::StreamExt;
mod args;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::try_parse()?;

    logger::init(args.no_color, args.log_level_filter())?;

    tokio::select! {
        _ = async {
            let mut signals = Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
            let handle = signals.handle();
            let _ = signals.next().await;
            handle.close();
            Ok::<_, anyhow::Error>(())
        } => {
            eprintln!("Termination signal received, stopping server...");
        },
        res = async move {
            let addr = args.socket_addr()?;
            eprintln!("Start server. addr: {:?}", &addr);
            server::serve(addr).await?;
            Ok::<_, anyhow::Error>(())
        } => {
            eprintln!("Stop server");
            res?
        },
    }
    Ok(())
}
