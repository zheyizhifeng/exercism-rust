use time::{PrimitiveDateTime as DateTime, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
    // unimplemented!("What time is a gigasecond later than {}", start);
}
