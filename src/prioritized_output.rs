pub mod prioritized_output {
    pub struct PrioritizedOutput {
        periode_hours: u8,
        nr_elements: u8,
    }

    impl PrioritizedOutput {
        pub fn new(hours: u8, elements: u8) -> PrioritizedOutput {
            PrioritizedOutput { periode_hours: hours, nr_elements: elements }
        }

        pub fn to_output(&self, prices: &Vec<crate::tibber::tibber::TibberPrice>) -> Result<(), anyhow::Error> {
            let p = prices.clone();
            let mut a = p.chunks(self.periode_hours as usize).clone();
            loop {
                let b = a.next();
                if b.is_none() {
                    break;
                }
                let mut c: Vec<crate::tibber::tibber::TibberPrice> = b.unwrap().to_vec();
                c.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                c = c[0..self.nr_elements as usize].to_vec();
                for i in c {
                    println!("hour: {:?}, price: {:?}", i.timestamp, i.price);    
                }
                println!("---")
            }

            Ok(())
        }
    }
}