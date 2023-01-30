use std::{fs, path::PathBuf};

use angry::{parser::{cli_parse, Cli}, BAD, GOOD, INFO};
use clap::Parser;
use futures::StreamExt;
use reqwest::StatusCode;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the URL and HTTP method from the command line
    let cli = Cli::parse();
    cli_parse();
    fetch_url(cli.url, &cli.wordlist, cli.threads).await;
    Ok(())
}

fn read_urls_from_file(filename: &PathBuf) -> Vec<String> {
    // Read the file into a String
    let contents = fs::read_to_string(filename).unwrap();

    // Split the contents of the file by newline characters
    let directories = contents.split("\n").map(String::from).collect();

    // Return the list of directories
    directories
}

async fn fetch_url(base_url: String, wordlist: &PathBuf, threads: usize) {
    let directories = read_urls_from_file(&wordlist);

    let mut paths = vec![];

    // Iterate through the directories and append to the URL
    for directory in directories {
        let request = format!("{}/{}", base_url, directory);
        paths.push(request);
    }

    let client = reqwest::Client::new();

    let fetches = futures::stream::iter(paths.into_iter().map(|path| {  
        let client = &client;
        async move {
        match client.get(&path).send().await {
            // Here we match on status so we can handle each use case we may find useful. Not sure if this is the best way forward?
            Ok(resp) => match resp.status() {
                // Have to await on text if you want the content length of the webpage. Will help with filtering out different word counts. Not sure which ones are worth returning?
                StatusCode::OK => match resp.text().await {
                    Ok(text) => println!("\x1b[1;32m{}\x1b[0m Status: \x1b[1;32m{}\x1b[0m {:<33}  Content Length: {}", GOOD, "200 OK", &path, text.len()),
                    Err(e) => println!("error {}", e)

                }
                // Handle any status we may find useful
                StatusCode::TEMPORARY_REDIRECT => println!("URL:{} Status {}", &path, resp.status()),
                StatusCode::MOVED_PERMANENTLY => println!("URL:{} Status {}", &path, resp.status()),
                StatusCode::UNAUTHORIZED => println!("URL:{} Status {}", &path, resp.status()),
                StatusCode::FORBIDDEN => println!("\x1b[1;93m{}\x1b[0m URL:{:<28} Status {}", INFO, &path, resp.status()),
                StatusCode::NOT_FOUND => println!("\x1b[1;91m{}\x1b[0m URL:{:<28} Status {}", BAD, &path, resp.status()),
                _ => println!("something else {}", resp.status())
            }
            Err(e) => println!("error parsing URL {}", e)
        }}
    }))
    .buffer_unordered(threads)
    .collect::<Vec<()>>();

    println!("Waiting....");
    fetches.await;

    // let n_urls = urls.len();
    // let client = Client::new();
    // let bodies = stream::iter(urls).map(|url| {
    //     let client = &client;
    //     async move {
    //         let resp = client.get(url).send().await?;

    //         resp.bytes().await
    //     }
    // }).buffer_unordered(n_urls);
    // x.await?;
    // let out = x.await.unwrap();
    // let new_bodies = bodies.
    // bodies.for_each(|b| async {
    //     match b {
    //         Ok(b) => println!("Got {} bytes", b.len()),
    //         Err(e) => eprintln!("Got an error: {}", e)
    //     }
    // }).await;
}
