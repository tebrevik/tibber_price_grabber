pub mod tibber {
    use chrono::{DateTime, FixedOffset};
    use serde::ser::{Serialize, Serializer, SerializeStruct};

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
    
    impl Serialize for TibberPrice {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("TibberPrice",2)?;
            state.serialize_field("timestamp", &self.timestamp)?;
            state.serialize_field("price", &self.price)?;
            state.end()
        }
    }
}