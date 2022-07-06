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
    match result.await {
        Ok(res) => println!("response: {}", res),
        Err(e) => println!("error: {}", e),
    }
    
    //println!("response: {:?}",res);
}
