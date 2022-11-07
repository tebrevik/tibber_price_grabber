pub mod cloudevent_output {
    use cloudevents::binding::nats::{NatsCloudEvent};
    use cloudevents::{EventBuilder, EventBuilderV10};
    use chrono::Utc;
    use uuid::Uuid;
    use serde_json::json;
    
    pub struct CloudEventsNats {
        server: String,
        subject: String,
    }

    impl CloudEventsNats {

        pub fn new(server: String, subject: String) -> CloudEventsNats {
            CloudEventsNats { server: server, subject: subject }
        }

        pub fn to_output(&self, prices: &Vec<crate::tibber::tibber::TibberPrice>) -> Result<(), anyhow::Error>{
            let nc = nats::connect(&self.server).unwrap();

            let event = EventBuilderV10::new()
                .id(Uuid::new_v4().to_string())
                .source("tibber_price_grabber")
                .ty("tibber.price.information.event")
                .time(Utc::now())
                .data("application/json", json!(prices) )
                .build()?;

            let n_msg = NatsCloudEvent::from_event(event).unwrap();
            
            nc.publish(&self.subject, n_msg)?;

            Ok(())
        }
    }
}