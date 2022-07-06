//use std::io::{self, Write};
use reqwest;
//use curl::easy::{Easy,List};

pub async fn get_access_token() -> Result<String,reqwest::Error>{
    let secret = "INSERT_TOKEN_HERE";
    let client = reqwest::Client::new();
    let res = client.post("https://api.vasttrafik.se:443/token")
        .body("grant_type=client_credentials") // TODO: add &scope=<device_id>
        .header("Authorization", "Basic ".to_owned() + &secret)
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get a payload");

    println!("hello world");
    Ok(res)

}



