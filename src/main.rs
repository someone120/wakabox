use std::env;

use draw::draw;
use waka_apis::waka_api;

mod draw;
mod waka_apis;

#[tokio::main]
async fn main() {
    let api_key = env::vars().find(|i|i.0=="API_KEY").unwrap();
    let stat=waka_api::get_stat(api_key.0.as_str()).await;
    println!("{}",draw(stat,50));
}

