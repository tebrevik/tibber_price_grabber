use anyhow::*;
use query::QueryViewerHomeCurrentSubscriptionPriceInfoToday;
use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use std::env;
use chrono::{Utc, DateTime, Timelike};
use chrono_tz::Europe::Oslo;

use crate::query::QueryViewerHomeCurrentSubscriptionPriceInfo;
use crate::query::QueryViewerHomeCurrentSubscriptionPriceInfoTomorrow;

mod terminal_output;
mod tibber;
use crate::tibber::tibber::TibberPrice;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/query.graphql",
    response_derives = "Debug"
)]
struct Query;

fn get_avg_max_and_min(data :Option<QueryViewerHomeCurrentSubscriptionPriceInfo>) {
    let today: &Vec<Option<QueryViewerHomeCurrentSubscriptionPriceInfoToday>> = data.as_ref().expect("today").today.as_ref();
    let tomorrow: &Vec<Option<QueryViewerHomeCurrentSubscriptionPriceInfoTomorrow>> = data.as_ref().expect("tomorrow").tomorrow.as_ref();
    let now = Utc::now();
    let mut avg: f64=0.0;
    let mut max: f64=0.0;
    let mut min: f64=200.0;
    let mut length = today.len() as f64;

    let mut prices: Vec<TibberPrice> = Vec::new();

    for hourly_info in today {
        let price = hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").total.unwrap();
        let hour = DateTime::parse_from_rfc3339(&hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").starts_at.as_ref().unwrap()).expect("no datetime");

        avg += price/length;
        if price < min {
            min = price;
        }
        if price > max {
            max = price;
        }
        if now.with_timezone(&Oslo).hour() <= hour.hour() {
            prices.push(TibberPrice{ timestamp: hour, price: price});
        }
    }

    println!("Dagens\t\tavg {:.3}\tmax {:.3}\tmin {:.3}",avg, max, min);

    avg = 0.0;
    min = 200.0;
    max = 0.0;
    length = tomorrow.len() as f64;
    for hourly_info in tomorrow {
        let price = hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").total.unwrap();
        let hour = DateTime::parse_from_rfc3339(&hourly_info.as_ref().expect("missing QueryViewerHomeCurrentSubscriptionPriceInfoToday data").starts_at.as_ref().unwrap()).expect("no datetime");

        avg += price/length;
        if price < min {
            min = price;
        }
        if price > max {
            max = price;
        }
        prices.push(TibberPrice{ timestamp: hour, price: price});
    }
    if length > 0.0 {
        println!("Morgendagens\tavg {:.3}\tmax {:.3}\tmin {:.3}",avg, max, min);
    } 
    terminal_output::terminal_output::to_output(prices);
}

fn get_today_prices(tibber_token: &str, home_id:&str) -> Result<(), anyhow::Error> {
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
    get_avg_max_and_min(data);

    Ok(())
}



fn main() -> Result<(), anyhow::Error> {
    let token = env::var("TIBBER_TOKEN")?;
    let home_id = env::var("TIBBER_HOME_ID")?;
 
    get_today_prices(token.as_str(), home_id.as_str())?;
    Ok(())
}
