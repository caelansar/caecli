use chrono::{Local, TimeZone, Utc};
use chrono_tz::Tz;

use crate::cmd::{Time, TimeUnit};

pub fn process_time(time: Time, timezone: Option<Tz>) -> anyhow::Result<()> {
    let dt = match time.unit {
        TimeUnit::Seconds => Utc.timestamp_opt(time.timestamp as i64, 0),
        TimeUnit::Milliseconds => Utc.timestamp_millis_opt(time.timestamp as i64),
    }
    .single()
    .ok_or_else(|| anyhow::anyhow!("invalid datetime"))?;

    let formatted_dt = match timezone {
        Some(tz) => dt.with_timezone(&tz).to_string(),
        None => dt.with_timezone(&Local).to_string(),
    };
    println!("{}", formatted_dt);

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone, Utc};

    #[test]
    fn test_timestamp() {
        let dt = Utc.timestamp_millis_opt(1714888305000).unwrap();
        println!("{}", dt.timestamp());

        let dt = dt.with_timezone(&Local).to_string();
        println!("{}", dt);
    }
}
