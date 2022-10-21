pub mod terminal_output {
    pub fn to_output(prices: &Vec<crate::tibber::tibber::TibberPrice>) {
        for i in prices {
            println!("hour: {:?}, price: {:?}", i.timestamp, i.price);
        }
    }
}