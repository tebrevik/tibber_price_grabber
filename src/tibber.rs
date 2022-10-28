pub mod tibber {
    use anyhow::*;
    use chrono::{DateTime, FixedOffset};
    use query::QueryViewerHomeCurrentSubscriptionPriceInfoToday;
    use ::reqwest::blocking::Client;
    use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
    
    #[derive(GraphQLQuery)]
    #[graphql(
        schema_path = "src/schema.graphql",
        query_path = "src/query.graphql",
        response_derives = "Debug"
    )]
    struct Query;

    use crate::tibber::tibber::query::QueryViewerHomeCurrentSubscriptionPriceInfo;
    use crate::tibber::tibber::query::QueryViewerHomeCurrentSubscriptionPriceInfoTomorrow;
    use serde::ser::{Serialize, Serializer, SerializeStruct};

    pub struct TibberPrice {
        pub timestamp: DateTime<FixedOffset>,
        pub price: f64,
    }

    pub struct TibberAttributes {
        pub date: chrono::NaiveDate,
        pub avg: f64,
        pub max: f64,
        pub min: f64,
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
    
    pub fn get_avg_max_and_min(prices: &Vec<TibberPrice>) -> Result<Vec<TibberAttributes>, anyhow::Error> {
        let mut attr: Vec<TibberAttributes> = Vec::new();
        let mut avg = 0.0;
        let mut min = 255.0;
        let mut max = 0.0;
        let mut current_date: Option<DateTime<FixedOffset>> = None;
        let mut nr_samples: u8 = 0;
        for i in prices {
            if current_date.is_none() {
                current_date = Some(i.timestamp);
                avg = i.price;
                max = i.price;
                min = i.price;
                nr_samples = 1;
            }
            else {
                if current_date.unwrap().date_naive() != i.timestamp.date_naive() {
                    avg /= nr_samples as f64;
                    attr.push(TibberAttributes{date: current_date.unwrap().date_naive(), avg: avg, max:max, min:min});
                    current_date = Some(i.timestamp);
                    avg = i.price;
                    max = i.price;
                    min = i.price;
                    nr_samples = 1;
                }
                else {
                    avg += i.price;
                    if max < i.price { max = i.price;}
                    if min > i.price { min = i.price;}
                    nr_samples += 1;
                }
            }
        }
        avg /= nr_samples as f64;
        attr.push(TibberAttributes{date: current_date.unwrap().date_naive(), avg: avg, max:max, min:min});
    Ok(attr)
    }
    
    fn to_tibber_vec(data :Option<QueryViewerHomeCurrentSubscriptionPriceInfo>) -> Result<Vec<TibberPrice>, anyhow::Error> {
        let today: &Vec<Option<QueryViewerHomeCurrentSubscriptionPriceInfoToday>> = data.as_ref().expect("today").today.as_ref();
    
        let mut prices: Vec<TibberPrice> = Vec::new();
    
        for hourly_info in today {
            let price = hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").total.unwrap();
            let ts = DateTime::parse_from_rfc3339(&hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").starts_at.as_ref().unwrap()).expect("no datetime");
    
            prices.push(TibberPrice{ timestamp: ts, price: price});
        }
    
        let tomorrow: &Vec<Option<QueryViewerHomeCurrentSubscriptionPriceInfoTomorrow>> = data.as_ref().expect("tomorrow").tomorrow.as_ref();
        for hourly_info in tomorrow {
            let price = hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").total.unwrap();
            let ts = DateTime::parse_from_rfc3339(&hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").starts_at.as_ref().unwrap()).expect("no datetime");
    
            prices.push(TibberPrice{ timestamp: ts, price: price});
        }
    
        return Ok(prices)
    }
    
    pub fn get_today_prices(tibber_token: &str, home_id:&str) -> Result<Vec<TibberPrice>, anyhow::Error> {
        let variables = query::Variables {
            id: home_id.to_string(),
        };
        let client = Client::builder()
            .user_agent("graphql-rust/0.10.0")
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", tibber_token))
                        .unwrap(),
                ))
                .collect(),
                )
            .build()?;
    
        let response_body: graphql_client::Response<query::ResponseData> = post_graphql::<Query, _>(&client, "https://api.tibber.com/v1-beta/gql", variables).unwrap();
        let data = response_body
                .data
                .expect("missing response data")
                .viewer
                .expect("missing QueryViewer data")
                .home
                .current_subscription
                .expect("missing QueryViewerHomeCurrentSubscription data")
                .price_info;
        to_tibber_vec(data)
    }
    
}