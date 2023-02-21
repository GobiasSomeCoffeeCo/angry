use std::{fs, io::stderr};

use angry::parser::{cli_parse, Config};
use angry::GOOD;

use anyhow::Ok;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let std_stderr = stderr();
    let config = cli_parse();
    config.print_banner(std_stderr)?;
    run(config).await;
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

async fn run(config: Config) {
    let directories = read_urls_from_file(&config.wordlist);
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let client = reqwest::Client::new();

    for directory in directories {
        let request = format!("{}/{}", config.url, directory);
        fetch_url(client.clone(), request, tx.clone());
    }

    drop(tx);

    // Default Status Codes: [200, 204, 301, 302, 307, 308, 401, 403, 405]
    while let Some(resp) = rx.recv().await {
        // Checks to see if exclude status codes was used. If not, either the default or user passed status codes will return.
        match config.exclude_status_codes.clone() {
            None => {
                if config.status_codes.contains(&resp.status().as_u16()) {
                    let (status, url) =(resp.status().as_u16(), resp.url().clone());
                    let text = resp.text().await.expect("Unable to retrieve response text");
                    color_status(status, &url, text);

                }
            }
            Some(exclude) => {
                if !&exclude.contains(&resp.status().as_u16()) {
                    let (status, url) =(resp.status().as_u16(), resp.url().clone());
                    let text = resp.text().await.expect("Unable to retrieve response text");
                    color_status(status, &url, text);
                }
            }
        }
    }

    // let fetches = futures::stream::iter(paths.into_iter().map(|path| {
    //     let client = &client;
    //     async move {
    //         let resp = client.get(&path).send().await.expect("msg");
    //         let status = resp.status().clone().to_string();
    //         tx.send(resp);
    //         // match client.get(&path).send().await {
    //         //     // Here we match on status so we can handle each use case we may find useful. Not sure if this is the best way forward?
    //         //     Ok(resp) => match resp.status() {
    //         //         // Have to await on text if you want the content length of the webpage. Will help with filtering out different word counts. Not sure which ones are worth returning?
    //         //         StatusCode::OK => match resp.text().await {
    //         //             Ok(text) => println!(
    //         //                 "{GOOD} Status: {STATUS_OK:<28} {:<33}  Content Length: {}",
    //         //                 &path,
    //         //                 text.len()
    //         //             ),
    //         //             Err(e) => println!("error {e}"),
    //         //         },
    //         //         // Handle any status we may find useful
    //         //         StatusCode::TEMPORARY_REDIRECT => {
    //         //             println!("{INFO} Status: {STATUS_TEMP_REDIRECT:<28} {:<33}", &path)
    //         //         }
    //         //         StatusCode::MOVED_PERMANENTLY => {
    //         //             println!(
    //         //                 "{INFO} Status: {STATUS_MOVED_PERMANENTLY:<28} {:<33}",
    //         //                 &path
    //         //             )
    //         //         }
    //         //         StatusCode::UNAUTHORIZED => {
    //         //             println!("{INFO} Status: {STATUS_UNAUTHORIZED:<28} {:<33}", &path)
    //         //         }
    //         //         StatusCode::FORBIDDEN => {
    //         //             println!("{INFO} Status: {STATUS_FORBIDDEN:<28} {:<33}", &path)
    //         //         }
    //         //         // StatusCode::NOT_FOUND => {
    //         //         //     println!("{BAD} Status: {STATUS_NOTFOUND:<28} {:<33}", &path)
    //         //         // }
    //         //         _ => (),
    //         //     },
    //         //     Err(e) => println!("error parsing URL {e}"),
    //         // }
    //     }
    // }))
    // .buffer_unordered(threads);
    // .collect::<Vec<()>>();

    // println!("Waiting....");
    // fetches.await;

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

fn fetch_url(
    client: reqwest::Client,
    url: String,
    tx: tokio::sync::mpsc::UnboundedSender<reqwest::Response>,
) {
    tokio::spawn(async move {
        let resp = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10)) // TODO Keeps the code from hanging, but returns an ugly error to the user.
            .send()
            .await
            .expect("unable to fetch URL");

        tx.send(resp).expect("unable to send channel");
    });
}

fn color_status(status: u16, url: &reqwest::Url, text: String) {
    if status >= 400 {
        println!(
            "{} Status: \x1b[1;91m{:<5}\x1b[0m {:<33} Content Length: {}",
            GOOD, status, url, text.len()
        )
    } else if (300..400).contains(&status) {
        println!(
            "{} Status: \x1b[1;93m{:<5}\x1b[0m {:<33} Content Length: {}",
            GOOD, status, url, text.len()
        )
    } else {
        println!(
            "{} Status: \x1b[1;32m{:<5}\x1b[0m {:<33} Content Length: {}",
            GOOD, status, url, text.len()
        )
    }
}
