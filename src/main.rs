use clap::Parser;
use colored::*;
use reqwest::Client;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The base URL to target
    #[clap(short, long)]
    url: String,

    /// Path to the wordlist file
    #[clap(short, long)]
    wordlist: String,

    /// Delay in seconds between requests - delay default set to 1 second so you don't hit rate limits.
    #[clap(short, long, default_value_t = 1)]
    delay: u64,
}

async fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let lines = buf
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>();
    Ok(lines)
}

async fn check_url(client: &Client, url: String) -> bool {
    match client.get(&url).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Client::new();
    let base_url = args.url;
    let wordlist_path = args.wordlist;
    let delay = args.delay;

    let lines = read_lines(wordlist_path).await?;
    let lines = Arc::new(lines);

    for line in lines.iter() {
        let url = format!("{}/{}", base_url, line);
        if check_url(&client, url.clone()).await {
            println!("{}", format!("Found: {}", url).green());
        } else {
            println!("{}", format!("Not Found: {}", url).red());
        }
        if delay > 0 {
            sleep(Duration::from_secs(delay)).await;
        }
    }

    Ok(())
}
