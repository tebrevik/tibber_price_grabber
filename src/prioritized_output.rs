pub mod prioritized_output {
    use std::cmp::Ordering;
    pub fn to_output(prices: &Vec<crate::tibber::tibber::TibberPrice>) {
        let mut p = prices.clone();
        p.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        println!("*****************");
        for i in p {
            println!("hour: {:?}, price: {:?}", i.timestamp, i.price);
        }
    }
}