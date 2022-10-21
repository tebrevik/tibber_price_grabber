pub mod tibber {
    use chrono::{DateTime, FixedOffset};

    pub struct TibberPrice {
        pub timestamp: DateTime<FixedOffset>,
        pub price: f64,
    }

    impl Clone for TibberPrice {
        fn clone(&self) -> Self {
            TibberPrice {
                timestamp: self.timestamp.clone(),
                price: self.price.clone(),
            }
        }
    }
}