mod api;

use clap::Parser;
use chrono::*;
use chrono_tz::Europe::Stockholm;

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
        Ok(_v) => Ok(()),
        Err(_e) => Err(String::from("Incorrect date format, date must be in YYYY-MM-DD format. Please try again.")),
    }

}

fn validate_time(time: &str) -> Result<(), String>{
    match NaiveTime::parse_from_str(time, "%H:%M"){
        Ok(_v) => Ok(()),
        Err(_e) => Err(String::from("Incorrect time format, time must be in HH:MM format. Please try again.")),
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

fn display_information(departures: Vec<api::Departure>, time: &String, date: &String) {
    let stop = &departures[0].stop;
    let current_time = NaiveTime::parse_from_str(time, "%H:%M").unwrap();
    println!("Departures: {} at {} on {}",stop, time, date);
    println!("-----------------------------------------------------------------");
    println!("Line           Destination           Next(min)           Location");
    println!("-----------------------------------------------------------------");

    for departure in departures{
        let line = departure.name.to_owned();
        let split_destination: Vec<&str> = departure.direction.split(",").collect();
        let destination = split_destination[0].to_string();
        let mut next;

        let track = &departure.track;
        let departure_time = NaiveTime::parse_from_str(&departure.time, "%H:%M").unwrap();

        // Sometimes departures with a departure time before the supplied time is returned.
        // These are probably delayed departures, skipping these. Might support in future version.
        if departure_time < current_time{
            continue;
        }

        let line_width = 15;
        let dest_width = 22;
        let mut next_width = 20;

        let next_int = (departure_time - current_time)
            .num_minutes()
            .rem_euclid(1440)
            .to_owned();

        if next_int == 60 {
            next = "1h".to_string();
        } else if next_int > 60 {
            let hours = (departure_time - current_time)
                .num_hours()
                .rem_euclid(24)
                .to_string();
            let minutes = next_int
                .rem_euclid(60)
                .to_string();
            next = format!("{}h{}m",hours,minutes);
        } else if next_int == 0 { //TODO: check if this gets the wheelchair emoji
            next = "Now ".to_string();
        } else {
            next = next_int.to_string();
        }

        let accessibility = departure.accessibility.to_owned();
        if Some("wheelChair".to_owned()) == accessibility {
            let wheelchair_emoji: &str = " \u{267F}";
            next = format!("{}{}", next, wheelchair_emoji);
            next_width = 19; // The wheelchair emoji takes one char in the and needed this for the padding to be correct.
        }

        println!("{line:line_width$}{destination:dest_width$}{next:next_width$}{}",track);

    }

}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let station = args.station;
    let dt_utc = Utc::now();
    let dt = dt_utc.with_timezone(&Stockholm);

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

    let location_id = match api::get_location_id(&token, &station).await {
        Ok(id) => id,
        Err(e) => panic!("error: {}", e),
    };

    let departures = match api::get_departure_board(&token,location_id,&date , &time).await{
        Ok(res) => res,
        Err(e) => panic!("error: {}", e),
    };

    display_information(departures,&time,&date);

}
