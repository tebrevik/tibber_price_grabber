pub mod terminal_output {
    use chrono::Timelike;
    use chrono::Utc;
    use chrono_tz::Europe::Oslo;

    pub fn to_output(prices: &Vec<crate::tibber::tibber::TibberPrice>) -> Result<(), anyhow::Error> {
        let now = Utc::now();

        for i in prices {
            if now.with_timezone(&Oslo).date_naive() < i.timestamp.date_naive() || now.with_timezone(&Oslo).hour() <= i.timestamp.hour() {
                println!("hour: {:?}, price: {:?}", i.timestamp, i.price);
            }
        }
        Ok(())
    }
}