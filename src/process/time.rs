use chrono::{Local, MappedLocalTime, TimeZone, Utc};

use crate::cmd::{Time, TimeUnit};

pub fn process_time(time: Time) -> anyhow::Result<()> {
    let dt = match time.unit {
        TimeUnit::Seconds => Utc.timestamp_opt(time.timestamp as i64, 0),
        TimeUnit::Milliseconds => Utc.timestamp_millis_opt(time.timestamp as i64),
    };

    let dt = match dt {
        MappedLocalTime::Single(dt) => dt,
        _ => anyhow::bail!("invalid datetime"),
    };

    let dt = dt.with_timezone(&Local).to_string();
    println!("{}", dt);

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
