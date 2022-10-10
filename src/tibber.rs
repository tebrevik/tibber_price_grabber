pub mod tibber {
    use chrono::{DateTime, FixedOffset};

    pub struct TibberPrice {
        pub timestamp: DateTime<FixedOffset>,
        pub price: f64,
    }
}