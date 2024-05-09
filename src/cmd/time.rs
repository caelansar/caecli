use clap::Parser;
use itertools::Itertools;

use crate::CmdExector;

#[derive(Debug, Parser)]
pub struct TimeOpts {
    #[arg(short, long, value_parser = verify_timestamp)]
    pub timestamp: Time,
    // TODO: timezone
}

impl CmdExector for TimeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_time(self.timestamp)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TimeUnit {
    Milliseconds,
    Seconds,
}

#[derive(Clone, Copy, Debug)]
pub struct Time {
    pub timestamp: u64,
    pub unit: TimeUnit,
}

fn verify_timestamp(ts: &str) -> anyhow::Result<Time> {
    let mut chars = ts.chars();
    let timestamp = chars.take_while_ref(|x| x.is_numeric());
    let timestamp = timestamp.collect::<String>();

    let unit = match chars.as_str() {
        "s" => TimeUnit::Seconds,
        "ms" => TimeUnit::Milliseconds,
        s => anyhow::bail!("invalid time unit `{}`", s),
    };

    let timestamp: u64 = timestamp.parse()?;

    Ok(Time { timestamp, unit })
}
