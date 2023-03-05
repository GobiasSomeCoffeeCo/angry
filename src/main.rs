use std::{fs, io::stderr};

use angry::client::create_client;
use angry::config::Config;
use angry::parser::cli_parse;
use angry::response::AngryResponse;
use angry::{BAD, GOOD};

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
    let contents = fs::read_to_string(filename).expect(
        "Unable to find the wordlist directory. Please make sure you provided the correct path",
    );

    let directories = contents.split('\n').map(String::from).collect();

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

    for directory in directories {
        if config.fuzz.is_some() {
            if let Some(user_input_url) = config.fuzz.clone() {
                let updated_url = user_input_url.replace("FUZZ", directory.trim_end());
                let parsed_url = reqwest::Url::parse(&updated_url)
                    .expect("Unable to parse URL. Please make sure the URL is formatted correctly");
                fetch_url(client.clone(), parsed_url.to_string(), tx.clone())
            }
        } else {
            let request = format!("{}/{}", config.url, directory.trim_end());
            fetch_url(client.clone(), request, tx.clone());
        }
    }

    drop(tx);

    // Default Status Codes: [200, 204, 301, 302, 307, 308, 401, 403, 405]
    while let Some(resp) = rx.recv().await {
        // TODO Add proper method handling
        let angry_response = AngryResponse::from(resp, "GET").await;
        // Checks to see if exclude status codes was used. If not, either the default or user passed status codes will return.
        match config.exclude_status_codes.clone() {
            None => {
                if config
                    .status_codes
                    .contains(&angry_response.status().as_u16())
                {
                    http_response_color_status(&angry_response);
                }
            }
            Some(exclude) => {
                if !&exclude.contains(&angry_response.status().as_u16()) {
                    http_response_color_status(&angry_response);
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
        let resp = client.get(&url).send().await;

        // Returns better user facing error handling for timed out responses.
        if let Err(e) = &resp {
            if e.is_timeout() {
                if let Some(z) = e.url() {
                    eprintln!("{} Request Timed Out --> {}", BAD, z);
                    return;
                }
            }
        }

        if let Err(e) = &resp {
            if e.is_connect() {
                // TODO Add logging so users can see this error if the choose to.
                return;
                // if let Some(error_url) = e.url() {
                //     // eprintln!("{} Unable to connect --> {}", BAD, error_url);
                //     return;
                // }
            }
        }

        // Handle all other errors here. Probably a better way to accomplish this.
        let resp = resp.expect("error while requesting URL");

        tx.send(resp).expect("unable to send channel");
    });
}

fn http_response_color_status(resp: &AngryResponse) {
    if resp.status().as_u16() >= 400 {
        println!(
            "{} Status: \x1b[1;91m{:<5}\x1b[0m {:<33} WC: {:<6} LC: {:<6} Content Length: {} ",
            GOOD,
            resp.status().as_u16(),
            resp.url(),
            resp.word_count(),
            resp.line_count(),
            resp.content_length()
        )
    } else if (300..400).contains(&resp.status().as_u16()) {
        println!(
            "{} Status: \x1b[1;93m{:<5}\x1b[0m {:<33} WC: {:<6} LC: {:<6} Content Length: {}",
            GOOD,
            resp.status().as_u16(),
            resp.url(),
            resp.word_count(),
            resp.line_count(),
            resp.content_length()
        )
    } else {
        println!(
            "{} Status: \x1b[1;32m{:<5}\x1b[0m {:<33} WC: {:<6} LC: {:<6} Content Length: {}",
            GOOD,
            resp.status().as_u16(),
            resp.url(),
            resp.word_count(),
            resp.line_count(),
            resp.content_length()
        )
    }
}
