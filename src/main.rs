use chrono::{DateTime, DurationRound, TimeDelta, TimeZone, Utc};
use nostr_sdk::prelude::*;
use rand::Rng;
use ross_ulbricht_freedom_bot::{
    DayOneContext, DayOneDelta, BEFORE_INAUGURATION_TEMPLATES, D1D, D1H_UTC, D1M, D1M_UTC, D1S_UTC,
    D1Y, RELAYS, SECONDS_IN_A_DAY, SECONDS_IN_TWO_DAYS,
};
use std::ops::Sub;

use log::{debug, info}; // Use log crate when building application
use secret::BECH32_NSEC;

pub mod secret;

#[tokio::main]

pub async fn main() -> Result<()> {
    env_logger::init();
    let secret_key = SecretKey::from_bech32(BECH32_NSEC)?;

    let live_posting = match std::env::args()
        .nth(1)
        .unwrap_or(String::from("false"))
        .as_str()
    {
        "false" | "0" => false,
        "true" | "1" => true,
        _ => false,
    };

    let current_date = match std::env::args().nth(2) {
        Some(timestamp) => {
            debug!("Timestamp supplied from CLI: {}", &timestamp);
            DateTime::<Utc>::from_timestamp(
                timestamp
                    .parse::<i64>()
                    .expect("Invalid CLI timestamp format"),
                0,
            )
            .unwrap()
        }
        None => Utc::now(),
    };

    run_bot(secret_key, live_posting, current_date).await
}

pub async fn run_bot(
    key: SecretKey,
    live_posting: bool,
    current_date: DateTime<Utc>,
) -> Result<()> {
    let day_one: DateTime<Utc> = Utc
        .with_ymd_and_hms(D1Y, D1M, D1D, D1H_UTC, D1M_UTC, D1S_UTC)
        .unwrap();

    let day_one_delta = get_time_delta(current_date, day_one);
    let day_one_context = determine_day_one_context(&day_one_delta);

    let keys = Keys::new(key);
    let client = Client::new(keys);
    add_relays(&client).await;
    client.connect().await;

    debug!("Client connected.");

    let message_text = format_template(
        select_random_template(day_one_context),
        day_one_delta.days,
        day_one_delta.hours,
        day_one_delta.minutes,
        day_one_delta.seconds,
    );
    info!("");
    info!("message text");
    info!("############");
    info!("\n{}", &message_text);
    info!("############");
    if live_posting {
        let builder = EventBuilder::text_note(&message_text);
        let output = client.send_event_builder(builder).await?;
        info!("posted to event id: {:?}", output.to_nostr_uri()?);
    }

    Ok(())
}

fn determine_day_one_context(delta: &DayOneDelta) -> DayOneContext {
    let total_seconds = delta.to_seconds();
    let before_seconds = SECONDS_IN_TWO_DAYS;
    let inauguration_seconds = SECONDS_IN_A_DAY;

    match delta {
        _d if total_seconds > before_seconds => DayOneContext::Before(delta),
        _d if total_seconds > inauguration_seconds => DayOneContext::InaugurationDay(delta),
        d if d.days > 0 => DayOneContext::DayOf(delta),
        d if d.days <= 0 => DayOneContext::After(delta),
        _ => DayOneContext::After(delta),
    }
}

fn get_time_delta(current_date: DateTime<Utc>, target_date: DateTime<Utc>) -> DayOneDelta {
    let date_format = "%Y-%m-%d %H:%M:%S %.3f";
    let current_date = current_date
        .duration_round(TimeDelta::try_minutes(1).unwrap())
        .unwrap();

    debug!("target: {}", target_date.format(date_format));
    debug!("current date: {}", current_date.format(date_format));

    let diff = target_date.sub(&current_date);
    let days = diff.num_days();
    let hours = diff.num_hours() - (&days * 24);
    let minutes = diff.num_minutes() - (&days * 24 * 60) - (&hours * 60);
    let seconds = diff.num_seconds() - (&days * 24 * 60 * 60) - (&hours * 60 * 60) - (minutes * 60);

    debug!(
        "delta values: {:0>2}:{:0>2}:{:0>2}:{:0>2}",
        days, hours, minutes, seconds
    );
    return DayOneDelta {
        days,
        hours,
        minutes,
        seconds,
    };
}

async fn add_relays(client: &Client) {
    for r in RELAYS {
        let _ = client.add_relay(*r).await;
    }
}

fn select_random_template(_context: DayOneContext) -> &'static str {
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..BEFORE_INAUGURATION_TEMPLATES.len());
    return BEFORE_INAUGURATION_TEMPLATES[idx];
}

fn format_template(
    template: &'static str,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
) -> String {
    let template_string = String::from(template);
    template_string
        .replace("<DAYS>", &days.to_string())
        .replace("<HOURS>", &hours.to_string())
        .replace("<MINUTES>", &minutes.to_string())
        .replace("<SECONDS>", &seconds.to_string())
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn get_day_one() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(D1Y, D1M, D1D, D1H_UTC, D1M_UTC, D1S_UTC)
            .unwrap()
    }

    #[test_log::test]
    fn get_time_delta_returns_delta_rounded_to_nearest_second() {
        let target_date = get_day_one();
        let current_date = target_date
            - Duration::days(1)
            - Duration::hours(1)
            - Duration::minutes(1)
            - Duration::seconds(1)
            - Duration::milliseconds(501);
        let expected = DayOneDelta {
            days: 1,
            hours: 1,
            minutes: 1,
            seconds: 2,
        };
        let actual = get_time_delta(current_date, target_date);
        assert_eq!(expected, actual);
    }

    #[test_log::test]
    fn determine_day_one_context_earlier_than_two_days_returns_before() {
        let day_one_end = get_day_one();
        let delta = get_time_delta(
            day_one_end.sub(Duration::days(2)) - Duration::milliseconds(501),
            day_one_end,
        );
        let result = determine_day_one_context(&delta);
        assert_eq!(result, DayOneContext::Before(&delta))
    }

    #[test_log::test]
    fn determine_day_one_context_two_days_returns_inauguration_day() {
        let day_one_end = get_day_one();
        let delta = get_time_delta(day_one_end.sub(Duration::days(2)), day_one_end);
        let result = determine_day_one_context(&delta);
        assert_eq!(result, DayOneContext::InaugurationDay(&delta))
    }

    #[test_log::test]
    fn determine_day_one_context_one_day_returns_day_of() {
        let day_one_end = get_day_one();
        let delta = get_time_delta(day_one_end.sub(Duration::days(1)), day_one_end);
        let result = determine_day_one_context(&delta);
        assert_eq!(result, DayOneContext::DayOf(&delta))
    }

    #[test_log::test]
    fn determine_day_one_context_zero_or_less_returns_after() {
        let day_one_end = get_day_one();
        let delta = get_time_delta(day_one_end, day_one_end.sub(Duration::days(0)));
        let result = determine_day_one_context(&delta);
        assert_eq!(result, DayOneContext::After(&delta))
    }

    #[test_log::test]
    fn from_timestamp_experiment() {
        let timestamp = 1737246274_i64;
        let dt = DateTime::<Utc>::from_timestamp(timestamp, 0).expect("invalid timestamp");
        debug!("dt: {:?}", dt);
        assert!(true)
    }
}
