pub mod cloudevent_output {
    use async_nats;
    use cloudevents::binding::nats::NatsCloudEvent;
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
            let rt = tokio::runtime::Runtime::new().unwrap();
            let nc = rt.block_on(async_nats::connect(&self.server)).unwrap();

            let event = EventBuilderV10::new()
                .id(Uuid::new_v4().to_string())
                .source("tibber_price_grabber")
                .ty("tibber.price.information.event")
                .time(Utc::now())
                .data("application/json", json!(prices) )
                .build()?;

            let n_msg = NatsCloudEvent::from_event(event).unwrap();
            rt.block_on(nc.publish(self.subject.clone(), n_msg.payload.into())).unwrap();
            rt.block_on(nc.flush())?;
            Ok(())
        }
    }
}
