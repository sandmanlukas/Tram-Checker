use serde::Deserialize;
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

//#[derive(Deserialize)]
//struct DepartureBoard {
//    departure: Vec<T>
//}

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

    println!("token: {}, expires: {}", access_token, expires_in);
    Ok(access_token)

}


//https://api.vasttrafik.se/bin/rest.exe/v2/departureBoard?id=test&date=2022-07-08&time=22%3A02&format=json

pub async fn get_departure_board(token: String, id: String, date: String, time: String) -> Result<String, reqwest::Error>
{
    let client = reqwest::Client::new();

    let base_url = "https://api.vasttrafik.se/bin/rest.exe/v2/departureBoard?";
    let id_url = format!("id={}&",id);
    let date_url = format!("date={}&",date);
    let time_url = format!("time={}&",time);
    let format_url = "format=json";
    let url = format!("{}{}{}{}{}",base_url,id_url,date_url,time_url,format_url);

    println!("url: {}", url);
    let res = client.get(&url)
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send()
        .await?
        .text() // TODO: change this to json.
        .await?;


    println!("{:?}",res);
    Ok("test".to_string())

}






