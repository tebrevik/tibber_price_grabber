pub mod prioritized_output {

    pub fn to_output(prices: &Vec<crate::tibber::tibber::TibberPrice>) -> Result<(), anyhow::Error> {
        let mut p = prices.clone();
        p.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        println!("***Prioritized start***");
        for i in p {
            println!("hour: {:?}, price: {:?}", i.timestamp, i.price);
        }
        println!("***Prioritized end***");

        Ok(())
    }
}