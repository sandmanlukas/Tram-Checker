//use serde::Deserialize;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use reqwest;
use std::env;
//use curl::easy::{Easy,List};


// Checks whether Västtrafik auth token is in environment variables.
fn get_env_token() -> String{
    let token = "VASTTRAFIK_TOKEN";

    match env::var(token) {
        Ok(v) => v,
        Err(e) => panic!("\nERROR: {} is not set. VASTTRAFIK_TOKEN should be set to the base64 encoding of key:secret as one of your environment variables.\nThe key and secret can be found at https://developer.vasttrafik.se/portal/#/applications", token)
    }


}

#[derive(Deserialize)]
struct Vasttrafik {
    access_token: String,
    expires_in: u32
}


pub async fn get_access_token() -> Result<String,reqwest::Error>{
    let secret = get_env_token();
    let client = reqwest::Client::new();
    let res = client.post("https://api.vasttrafik.se:443/token")
        .body("grant_type=client_credentials") // TODO: add &scope=<device_id>
        .header("Authorization", "Basic ".to_owned() + &secret)
        .send()
        .await?
        .json::<Vasttrafik>()
        .await?;

    let access_token = res.access_token;
    let expires_in = res.expires_in; //TODO: add code to check if token is about to expire, <60s left?

    //println!("token: {}, expires: {}", access_token, expires_in);
    Ok(access_token)

}




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartureRoot {
    #[serde(rename = "DepartureBoard")]
    pub departure_board: DepartureBoard,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartureBoard {
    #[serde(rename = "Departure")]
    pub departure: Vec<Departure>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    pub name: String,
    pub sname: String,
    pub journey_number: String,
    #[serde(rename = "type")]
    pub type_field: String,
    //pub stopid: String,
    pub stop: String,
    pub time: String,
    pub date: String,
    //pub journeyid: String,
    pub direction: String,
    pub track: String,
    //pub rt_time: Option<String>,
    //pub rt_date: Option<String>,
    //pub fg_color: String,
    //pub bg_color: String,
    //pub stroke: String,
    pub accessibility: Option<String>,
    //#[serde(rename = "JourneyDetailRef")]
    //pub journey_detail_ref: JourneyDetailRef,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationRoot {
    #[serde(rename = "LocationList")]
    pub location_list: LocationList,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationList {
    pub no_namespace_schema_location: String,
    pub servertime: String,
    pub serverdate: String,
    #[serde(rename = "StopLocation")]
    pub stop_location: Option<Vec<StopLocation>>,
    #[serde(rename = "CoordLocation")]
    pub coord_location: Option<Vec<CoordLocation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopLocation {
    pub name: String,
    pub lon: String,
    pub lat: String,
    pub id: String,
    pub idx: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordLocation {
    pub name: String,
    pub lon: String,
    pub lat: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub idx: String,
}


pub async fn get_departure_board(token: &String, id: String, date: &String, time: &String) -> Result<Vec<Departure>, reqwest::Error>
{
    let client = reqwest::Client::new();

    let base_url = "https://api.vasttrafik.se/bin/rest.exe/v2/departureBoard?";
    let id_url = format!("id={}&",id);
    let date_url = format!("date={}&",date);

    let time_url = format!("time={}&",time);
    let format_url = "format=json";
    let url = format!("{}{}{}{}{}",base_url,id_url,date_url,time_url,format_url);

    let res = client.get(&url)
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await?;
    let status = res.status();

    if !status.is_success(){
        panic!("Something went wrong! Status: {:?}", status);
    }

    let res_json = res.json::<DepartureRoot>().await?;
    let departures = res_json.departure_board.departure;
    Ok(departures)
}

//https://api.vasttrafik.se/bin/rest.exe/v2/location.name?input=Chalmers&format=json


// The API call pattern matches on the inputted stop name and tries to find the best possible match
// This function returns the id of most probable match, returns an error if no possible matches are returned.
pub async fn get_location_id(token: &String, stop: &String) -> Result<String, reqwest::Error>{
    let client = reqwest::Client::new();

    let base_url = "https://api.vasttrafik.se/bin/rest.exe/v2/location.name?";
    let stop_url = format!("input={}&", stop);
    let format_url = "format=json";
    let url = format!("{}{}{}", base_url, stop_url, format_url);
    
    let res = client.get(&url)
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await?;

    let status = res.status();

    if !status.is_success(){
        panic!("Something went wrong! Status: {:?}", status);
    }

    let res_json = res.json::<LocationRoot>().await?;

    let stop_locations = &res_json.location_list.stop_location;

    let location_id = match stop_locations {
        Some(p) => p[0].id.to_string(),
        None => panic!("The stop {} doesn't exist. Try another one", stop),
    };

    //let location_id = &stop_locations[0].id.to_string();

    Ok(location_id)

}
