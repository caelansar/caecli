use chrono_tz::Tz;
use clap::Parser;
use itertools::Itertools;
use std::str::FromStr;

use crate::CmdExector;

#[derive(Debug, Parser)]
pub struct TimeOpts {
    #[arg(short, long, value_parser = verify_timestamp)]
    pub timestamp: Time,
    #[arg(long = "tz", value_parser = verify_timezone)]
    pub timezone: Option<Tz>,
}

impl CmdExector for TimeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        crate::process_time(self.timestamp, self.timezone)?;
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

fn verify_timezone(tz: &str) -> anyhow::Result<Tz> {
    let tz = Tz::from_str(tz)?;
    Ok(tz)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Tz;

    #[test]
    fn test_verify_timezone() {
        assert!(verify_timezone("UTC").is_ok());
        assert!(verify_timezone("America/New_York").is_ok());
        assert!(verify_timezone("Europe/London").is_ok());
        assert!(verify_timezone("Asia/Tokyo").is_ok());
        assert!(verify_timezone("Asia/Shanghai").is_ok());

        assert!(verify_timezone("Invalid/Timezone").is_err());
        assert!(verify_timezone("NotATimezone").is_err());
        assert!(verify_timezone("").is_err());
    }

    #[test]
    fn test_verify_timezone_case_sensitivity() {
        assert!(verify_timezone("utc").is_err());
        assert!(verify_timezone("america/new_york").is_err());
        assert!(verify_timezone("EUROPE/LONDON").is_err());
    }

    #[test]
    fn test_verify_timezone_returns_correct_tz() {
        let tz = verify_timezone("UTC").unwrap();
        assert_eq!(tz, Tz::UTC);

        let tz = verify_timezone("America/New_York").unwrap();
        assert_eq!(tz, Tz::America__New_York);
    }
}
