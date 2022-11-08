use std::env;
use clap::Parser;
use crate::prioritized_output::prioritized_output::PrioritizedOutput;
use crate::cloudevent_output::cloudevent_output::CloudEventsNats;

mod terminal_output;
mod prioritized_output;
mod cloudevent_output;
mod tibber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,

    #[arg(short,long, default_value_t = 9) ]
    periode_hours: u8,

    #[arg(short,long, default_value_t = 2) ]
    number_of_elements_prioritized: u8,

    #[arg(long, default_value_t = String::from("localhost:4222"))]
    server_nats: String,

    #[arg(long, default_value_t = String::from("tibber_prices"))]
    subject_nats: String,
}


fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let token = env::var("TIBBER_TOKEN")?;


    match args.mode.as_str() {
        "List" => {
            let home_id = env::var("TIBBER_HOME_ID")?;
            let res = crate::tibber::tibber::get_today_prices(token.as_str(), home_id.as_str())?;
                    terminal_output::terminal_output::to_output(res.as_ref())?;
            let attr = crate::tibber::tibber::get_avg_max_and_min(res.as_ref())?;
            for i in attr {
                println!("{:?} - avg: {:.3}, max: {:.3}, min: {:.3}", i.date,i.avg,i.max, i.min);
            }

        }
        "Priority" => {
            let home_id = env::var("TIBBER_HOME_ID")?;
            let res = crate::tibber::tibber::get_today_prices(token.as_str(), home_id.as_str())?;
                    let po = PrioritizedOutput::new(args.periode_hours,args.number_of_elements_prioritized);
            po.to_output(res.as_ref())?;
        }
        "CloudEvents" => {
            let home_id = env::var("TIBBER_HOME_ID")?;
            let res = crate::tibber::tibber::get_today_prices(token.as_str(), home_id.as_str())?;
                    let cen = CloudEventsNats::new(args.server_nats,args.subject_nats);
            cen.to_output(res.as_ref())?;
        }
        "ListHomes" => {
            let res = crate::tibber::tibber::get_homes(token.as_str());
            for home in res.unwrap() {
                let addr = home.address.unwrap_or(String::from("unknown"));
                let postal_code = home.postal_code.unwrap_or(String::from(""));
                let city = home.city.unwrap_or(String::from(""));
                let country = home.country.unwrap_or(String::from(""));
                println!("ID: {:?}, Address: {:?}, postal code: {:?}, city: {:?}, country: {:?}", home.id, addr, postal_code, city, country);
            }
        }
        _ => {println!("no mode specified (List, Priority or CloudEvents)");}
    }


    Ok(())
}
