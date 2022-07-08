#![allow(unused)]

mod api;

use clap::Parser;
//use anyhow::{Context, Result};

/// Show next departing trams/buses/etc from given station in Gothenburg.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The tram station to display info about
    station: String,

}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let station = args.station;
    println!("Station: {}", &station);


    let result = api::get_access_token();
    let token = match result.await {
        Ok(res) => res,
        Err(e) => panic!("error: {}", e),
    };

    let departure = match api::get_departure_board(token,"Chalmers".to_string(), "2022-07-09".to_string(), "12:12".to_string()).await{
        Ok(res) => res,
        Err(e) => panic!("error: {}", e),
    };

}
