use reqwest::StatusCode;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;

const CONCURRENCY_LIMIT: usize = 10;
const OUTPUT_FILE_NAME: &str = "discovered_files.txt";

async fn fetch_directory_entries(url: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Ok(Vec::new());
    }
    let text = response.text().await?;
    let mut entries = Vec::new();
    for line in text.lines() {
        let parts: Vec<&str> = line.split('"').collect();
        if parts.len() >= 2 {
            let path = PathBuf::from(parts[1]);
            entries.push(path);
        }
    }
    Ok(entries)
}

async fn explore_directory(
    base_url: String,
    path: PathBuf,
    sender: Sender<PathBuf>,
    quit: oneshot::Receiver<()>,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/{}", base_url, path.to_str().unwrap());
    let entries = fetch_directory_entries(&url).await?;
    for entry in entries {
        if entry.is_dir() {
            let sender_clone = sender.clone();
            let (quit_sender, quit_receiver) = oneshot::channel();
            tokio::spawn(async move {
                if let Err(e) =
                    explore_directory(base_url.clone(), entry, sender_clone, quit_receiver).await
                {
                    eprintln!("Error exploring directory {}: {}", entry.display(), e);
                }
            });
            tokio::spawn(async move {
                let _ = quit.await;
                let _ = quit_sender.send(());
            });
        } else {
            let _ = sender.send(entry).await;
        }
    }
    Ok(())
}

async fn write_discovered_files(mut receiver: Receiver<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut discovered_files = HashSet::new();
    while let Some(path) = receiver.recv().await {
        if discovered_files.insert(path.clone()) {
            println!("{}", path.display());
            let mut file = File::create(OUTPUT_FILE_NAME)?;
            writeln!(file, "{}", path.display())?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} URL", args[0]);
        std::process::exit(1);
    }
    let base_url = args[1].clone();
    let (sender, receiver) = mpsc::channel(CONCURRENCY_LIMIT);
    let (quit_sender, quit_receiver) = oneshot::channel();
    let writer_task = tokio::spawn(async move {
        if let Err(e) = write_discovered_files(receiver).await {
            eprintln!("Error writing discovered files: {}", e);
        }
    });
    let exploration_task = tokio::spawn(async move {
        if let Err(e) = explore_directory(base_url, PathBuf::new(), sender, quit_receiver).await {
            eprintln!("Error exploring directory {}: {}", base_url, e);
        }
    });
    tokio::select! {
        _ = exploration_task => {
            let _ = quit_sender.send(());
        }
        _ = writer_task => {}
    }
    let (mut discovered_urls_sender, mut discovered_urls_receiver) =
        mpsc::channel(CONCURRENCY_LIMIT);
    let urls_to_explore = HashSet::new();
    urls_to_explore.insert(base_url.clone());
    while let Some(url) = discovered_urls_receiver.recv().await {
        if !urls_to_explore.contains(&url) {
            urls_to_explore.insert(url.clone());
            let exploration_task = tokio::spawn(async move {
                if let Err(e) = explore_directory(
                    url.clone(),
                    PathBuf::new(),
                    discovered_urls_sender.clone(),
                    quit_receiver,
                )
                .await
                {
                    eprintln!("Error exploring directory {}: {}", url, e);
                }
            });
            tokio::spawn(async move {
                let _ = quit.await;
            });
        }
    }
    Ok(())
}
