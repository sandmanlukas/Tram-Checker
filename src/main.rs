#![allow(unused)]

mod api;

use clap::Parser;
use chrono::prelude::*;


fn validate_station(station: &str) -> Result<(), String> {
    if station.trim().len() != station.len() {
        Err(String::from(
            "Station cannot have leading and trailing space",
        ))
    } else {
        Ok(())
    }
}

fn validate_date(date: &str) -> Result<(), String>{
    match NaiveDate::parse_from_str(date, "%Y-%m-%d"){
        Ok(v) => Ok(()),
        Err(e) => Err(String::from("Incorrect date format, date must be in YYYY-MM-DD format. Please try again.")),
    }

}

fn validate_time(time: &str) -> Result<(), String>{
    match NaiveTime::parse_from_str(time, "%H:%M"){
        Ok(v) => Ok(()),
        Err(e) => Err(String::from("Incorrect time format, time must be in HH:MM format. Please try again.")),
    }

}

/// Show next departing trams/buses/etc from given station in Gothenburg.
#[derive(Parser, Debug, Default)]
#[clap(author="Lukas Sandman", version="1.0", about="A simple CLI-app to get information about coming trams and buses in Gothenburg.", long_about = None)]
struct Cli {
    #[clap(forbid_empty_values = true, validator=validate_station)]
    /// The tram station to display info about
    station: String,
    #[clap(short, long,validator=validate_time)]
    /// If specified, displays information on the coming trams/buses 20min from specified time. Time must be in HH:MM format. If not specified, uses the current time.
    time: Option<String>,
    #[clap(short,long, validator=validate_date)]
    /// If specified, displays information on the coming trams/buses on specified date. Date must be in YYYY-MM-DD format. If not specified, uses current date.
    date: Option<String>,


}


#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let station = args.station;
    let dt = Utc::now();

    let time = match args.time {
        Some(p) => p,
        None => dt.format("%H:%M").to_string(),
    };

    let date = match args.date {
        Some(p) => p,
        None => dt.format("%Y-%m-%d").to_string(),
    };

    let result = api::get_access_token();
    let token = match result.await {
        Ok(res) => res,
        Err(e) => panic!("error: {}", e),
    };

    let location_id = match api::get_location_id(&token, station
    ).await {
        Ok(id) => id,
        Err(e) => panic!("error: {}", e),
    };

    let departures = match api::get_departure_board(&token,location_id,date , time).await{
        Ok(res) => res,
        Err(e) => panic!("error: {}", e),
    };

    println!("departures: {:#?}", departures);

}


/*
Line Destination Next(min) Location
-----------------------------------
7    Nordstan      3 ♿       A
8    Brunnsparken  4         C
*/
