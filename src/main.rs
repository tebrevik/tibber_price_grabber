use anyhow::*;
use ::reqwest::blocking::Client;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use std::env;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/query.graphql",
    response_derives = "Debug"
)]
struct Query;

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
            .expect("response data")
            .viewer
            .expect("viewer")
            .home
            .current_subscription
            .expect("subsc")
            .price_info
            .expect("price")
            .today;

    for hourly_info in data {
        let price = hourly_info.as_ref().expect("hourly").total.unwrap();
        let hour = &hourly_info.as_ref().expect("hourly").starts_at.as_ref().unwrap();
        println!("pris: {:?},-\tstarter {:?}", price,hour);
    }

    Ok(())
}



fn main() -> Result<(), anyhow::Error> {
    let token = env::var("TIBBER_TOKEN")?;
    let home_id = env::var("TIBBER_HOME_ID")?;
 
    get_today_prices(token.as_str(), home_id.as_str())?;
    Ok(())
}
