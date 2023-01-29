use std::{fs, path::PathBuf};


use angry::parser::Cli;
use clap::{arg, Parser};
use futures::StreamExt;
use reqwest::StatusCode;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the URL and HTTP method from the command line
    // let url = get_url_from_command_line();
    let cli = Cli::parse();

    fetch_url(cli.url, &cli.wordlist).await;


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

async fn fetch_url(base_url: String, wordlist: &PathBuf) {
    let directories = read_urls_from_file(&wordlist);

    let mut paths = vec![];

    for directory in directories {
        let request = format!("{}/{}", base_url, directory);
        paths.push(request);
    }
    let client = reqwest::Client::new();

    let fetches = futures::stream::iter(paths.into_iter().map(|path| { 
        let client = &client;
         async move {
        match client.get(&path).send().await {
            Ok(resp) => match resp.status() {
                StatusCode::OK => match resp.text().await {
                    Ok(text) => println!("URL: {}, Status: 200 OK Content Length: {}", &path, text.len()),
                    Err(e) => println!("error {}", e)

                }
                StatusCode::FORBIDDEN => println!("URL:{} Status {}", &path, resp.status()),
                StatusCode::NOT_FOUND => println!("URL:{} Status {}", &path, resp.status()),
                _ => println!("something else")
            }
            Err(e) => println!("error parsing URL {}", e)
            // Ok(resp) => match resp.text().await {
            //     Ok(text) => {
            //         println!("URL: {} Response:Bytes: {}", path, text.len())
            //     }

            //     Err(_) => println!("ERROR reading {}", path),
            // },
            // Err(_) => println!("Error downloading path {}", path),
        }}
    }))
    .buffer_unordered(8)
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
