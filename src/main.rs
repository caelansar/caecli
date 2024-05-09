use caecli::{CmdExector, Opts};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    opts.cmd.execute().await?;

    Ok(())
}
