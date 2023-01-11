use std::{env, fs, io};

use draw::draw;
use waka_apis::waka_api;

mod draw;
mod waka_apis;

#[tokio::main]
async fn main() {
    let api_key = env::vars().find(|i| i.0 == "API_KEY").unwrap();
    let stat = waka_api::get_stat(api_key.1.as_str()).await;

    let md = fs::read_to_string("README.md").unwrap();

    let start_tag = "<!-- waka-box start -->";
    let index_start = md.find(start_tag).unwrap();
    let before = &md[0..index_start+(start_tag.len())];
    let end_tag = "<!-- waka-box end -->";
    let index_end = md.find(end_tag).unwrap();
    let after = &md[index_end..];

    let new_md = before.to_owned()+&draw(stat, 50)+after;

    println!("{}", new_md);
    fs::write("README.md", new_md).unwrap();
}
