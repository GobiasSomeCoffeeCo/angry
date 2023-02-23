use std::{fs, io::stderr};

use angry::client::create_client;
use angry::config::Config;
use angry::parser::cli_parse;
use angry::GOOD;

use angry::response::AngryResponse;
use anyhow::Ok;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let std_stderr = stderr();
    let config = cli_parse();
    config.print_banner(std_stderr)?;
    run(config).await?;
    Ok(())
}

fn read_urls_from_file(filename: &str) -> Vec<String> {
    // Read the file into a String
    let contents = fs::read_to_string(filename).expect(
        "Unable to find the wordlist directory. Please make sure you provided the correct path",
    );

    // Split the contents of the file by newline characters
    let directories = contents.split('\n').map(String::from).collect();

    // Return the list of directories
    directories
}

async fn run(config: Config) -> anyhow::Result<()> {
    let directories = read_urls_from_file(&config.wordlist);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let client = create_client(
        config.timeout.try_into().unwrap(),
        &config.user_agent,
        config.redirects,
        config.insecure,
        &config.headers,
        config.proxy,
    )?;
    // let client = reqwest::Client::new();

    for directory in directories {
        let request = format!("{}/{}", config.url, directory);
        fetch_url(client.clone(), request, tx.clone());
    }

    drop(tx);

    // Default Status Codes: [200, 204, 301, 302, 307, 308, 401, 403, 405]
    while let Some(resp) = rx.recv().await {
        // Checks to see if exclude status codes was used. If not, either the default or user passed status codes will return.
        let angry_response = AngryResponse::from(resp, "GET").await;
        match config.exclude_status_codes.clone() {
            None => {
                if config
                    .status_codes
                    .contains(&angry_response.status().as_u16())
                {
                    let (status, url) = (angry_response.status().as_u16(), angry_response.url());

                    let text = angry_response.text();

                    color_status(status, url, text);
                }
            }
            Some(exclude) => {
                if !&exclude.contains(&angry_response.status().as_u16()) {
                    let (status, url) = (angry_response.status().as_u16(), angry_response.url());
                    let text = angry_response.text();
                    color_status(status, url, text);
                }
            }
        }
    }

    Ok(())
}

fn fetch_url(
    client: reqwest::Client,
    url: String,
    tx: tokio::sync::mpsc::UnboundedSender<reqwest::Response>,
) {
    tokio::spawn(async move {
        let resp = client.get(&url).send().await.expect("unable to fetch URL");

        tx.send(resp).expect("unable to send channel");
    });
}

fn color_status(status: u16, url: &reqwest::Url, text: &str) {
    let (content_length, lc, wc) = get_text(text);
    if content_length == u64::MAX {
        println!("hello")
    }
    if status >= 400 {
        println!(
            "{} Status: \x1b[1;91m{:<5}\x1b[0m {:<33} WC: {:<6} LC: {:<6} Content Length: {} ",
            GOOD, status, url, wc, lc, content_length
        )
    } else if (300..400).contains(&status) {
        println!(
            "{} Status: \x1b[1;93m{:<5}\x1b[0m {:<33} WC: {:<6} LC: {:<6} Content Length: {}",
            GOOD, status, url, wc, lc, content_length
        )
    } else {
        println!(
            "{} Status: \x1b[1;32m{:<5}\x1b[0m {:<33} WC: {:<6} LC: {:<6} Content Length: {}",
            GOOD, status, url, wc, lc, content_length
        )
    }
}

fn get_text(text: &str) -> (u64, usize, usize) {
    let content_length = text.len() as u64;
    let line_count = text.lines().count();
    let word_count: usize = text.lines().map(|s| s.split_whitespace().count()).sum();

    (content_length, line_count, word_count)
}
