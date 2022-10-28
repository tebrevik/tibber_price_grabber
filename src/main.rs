use std::env;
use clap::Parser;
use crate::prioritized_output::prioritized_output::PrioritizedOutput;


mod terminal_output;
mod prioritized_output;
mod cloudevent_output;
mod tibber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,
}


fn main() -> Result<(), anyhow::Error> {
    let token = env::var("TIBBER_TOKEN")?;
    let home_id = env::var("TIBBER_HOME_ID")?;

    let args = Args::parse();
    let res = crate::tibber::tibber::get_today_prices(token.as_str(), home_id.as_str())?;

    match args.mode.as_str() {
        "List" => {
            terminal_output::terminal_output::to_output(res.as_ref())?;
            let attr = crate::tibber::tibber::get_avg_max_and_min(res.as_ref())?;
            for i in attr {
                println!("{:?} - avg: {:.3}, max: {:.3}, min: {:.3}", i.date,i.avg,i.max, i.min);
            }

        }
        "Priority" => {
            let po = PrioritizedOutput::new(9,2);
            po.to_output(res.as_ref())?;
        }
        "CloudEvents" => {
            cloudevent_output::cloudevent_output::to_output(res.as_ref())?;
        }
        _ => {println!("no mode specified (List, Priority or CloudEvents)");}
    }

    Ok(())
}
