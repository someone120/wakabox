use base64::{engine::general_purpose, Engine as _};
use reqwest::{Client, RequestBuilder};
use std::str;

use super::structs::Stats;

pub fn add_header(rb: RequestBuilder, api_key: &str) -> RequestBuilder {
    println!("{}", general_purpose::STANDARD.encode(api_key));
    rb.header(
        "Authorization",
        format!("Basic {}", general_purpose::STANDARD.encode(api_key)),
    )
    .header("Accept", "application/x-www-form-urlencoded")
}

async fn get_stat_raw(api_key: &str)->String {
    let client = Client::new();
    let mut rb = client.get("https://wakatime.com/api/v1/users/current/stats/last_7_days");
    rb = add_header(rb, api_key);

    let res = rb.send().await.unwrap();
    res.text().await.unwrap()
}

pub async fn get_stat(api_key: &str)->Stats{
    let stat=get_stat_raw(api_key).await;
    let result:Stats=serde_json::from_str(&stat[..]).unwrap();
    result
}