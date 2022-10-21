pub mod cloudevent_output {
    use cloudevents::binding::nats::{MessageExt, NatsCloudEvent};
    use cloudevents::{EventBuilder, EventBuilderV10};
    use chrono::Utc;
    use uuid::Uuid;
    use serde_json::json;
    
    pub fn to_output(prices: &Vec<crate::tibber::tibber::TibberPrice>) -> Result<(), anyhow::Error>{
        let event = EventBuilderV10::new()
            .id(Uuid::new_v4().to_string())
            .source("tibber_price_grabber")
            .ty("tibber.price.information.event")
            .time(Utc::now())
            .data("application/json", json!(prices) )
            .build()?;

        let n_msg = NatsCloudEvent::from_event(event);
        
        println!("{:?}",n_msg);

        Ok(())
    }
}