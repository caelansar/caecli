use clap::Parser;

#[derive(Debug, Parser)]
pub struct TimeOpts {
    #[arg(short, long, value_parser = verify_timestamp)]
    pub timestamp: Time,
    // TODO: timezone
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
    let timestamp = ts.chars().take_while(|x| x.is_numeric());
    let timestamp = timestamp.collect::<String>();

    let unit = match ts.strip_prefix(&timestamp) {
        Some("s") => TimeUnit::Seconds,
        Some("ms") => TimeUnit::Milliseconds,
        _ => anyhow::bail!("invalid time unit"),
    };

    let timestamp: u64 = timestamp.parse()?;

    Ok(Time { timestamp, unit })
}
