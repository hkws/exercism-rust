use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigaseconds = Duration::seconds(10i64.pow(9));
    start + gigaseconds
}
