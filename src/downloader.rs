use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use reqwest::blocking::Client;
use crate::progress::ProgressTracker;
use crate::utils;
use std::thread;

pub struct DownloadStats {
    pub start_time: String,
    pub end_time: String,
    pub url: String,
    pub status: String,
    pub content_length: u64,
    pub file_path: PathBuf,
    pub downloaded_size: u64,
}

pub fn download(args: crate::cli::CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let urls = if let Some(ref input_file) = args.input_file {
        utils::read_urls_from_file(input_file)?
    } else {
        vec![args.url.clone().ok_or("No URL provided")?.clone()]
    };

    if args.background {
        println!("Output will be written to 'wget-log'.");
    }

    // Create a log file if in background mode
    let mut log_file = if args.background {
        Some(File::create("wget-log")?)
    } else {
        None
    };

    for url in urls {
        let stats = download_single_file(&url, &client, &args)?;
        
        if let Some(ref mut file) = log_file {
            write!(file, "start at {}\n", stats.start_time)?;
            write!(file, "sending request, awaiting response... {}\n", stats.status)?;
            write!(file, "content size: {} [{}]\n", 
                stats.content_length,
                utils::format_size(stats.content_length as usize))?;
            write!(file, "saving file to: {}\n", stats.file_path.display())?;
            write!(file, "Downloaded [{}]\n", stats.url)?;
            write!(file, "finished at {}\n\n", stats.end_time)?;
        }
    }

    Ok(())
}

fn download_single_file(
    url: &str,
    client: &Client,
    args: &crate::cli::CliArgs,
) -> Result<DownloadStats, Box<dyn std::error::Error>> {
    let start_time = utils::get_current_time();
    
    if !args.background {
        println!("Starting download for {}", url);
        println!("Start time: {}", start_time);
    }

    // Send the HTTP request
    let response = client.get(url).send()?;
    let status = format!("{}", response.status());

    // Display the HTTP status
    if !args.background {
        println!("Response Status: {}", status);
    }

    // Log the HTTP status if in background mode
    if args.background {
        println!("{}: HTTP status: {}", start_time, status);
    }

    // Check for non-successful response
    if !response.status().is_success() {
        return Err(format!("Failed to download {}: {}", url, status).into());
    }

    let content_length = response.content_length().unwrap_or_default();
    let filename = args.get_output_path(url);

    // Create parent directories if they don't exist
    if let Some(parent) = filename.parent() {
        fs::create_dir_all(parent)?;
    }

    let progress = if !args.background {
        Some(ProgressTracker::new(content_length))
    } else {
        None
    };

    let mut file = File::create(&filename)?;
    let mut downloaded: u64 = 0;
    let rate_limit = args.parse_rate_limit();
    let mut last_update = Instant::now();

    for chunk in response.bytes()?.chunks(8192) {
        file.write_all(chunk)?;
        downloaded += chunk.len() as u64;

        if let Some(ref progress) = progress {
            progress.update(downloaded);
        }

        // Rate limiting
        if let Some(rate) = rate_limit {
            let elapsed = last_update.elapsed();
            let expected_time = Duration::from_secs_f64(chunk.len() as f64 / rate as f64);
            if elapsed < expected_time {
                thread::sleep(expected_time - elapsed);
            }
            last_update = Instant::now();
        }
    }

    if let Some(progress) = progress {
        progress.finish();
    }

    let end_time = utils::get_current_time();
    
    if !args.background {
        println!(
            "File saved to: {} [{}]",
            filename.display(),
            utils::format_size(downloaded as usize) // Use downloaded_size
        );
        println!("Downloaded size: {} bytes", downloaded); // Display raw size
        println!("End time: {}", end_time);
    }
    

    Ok(DownloadStats {
        start_time,
        end_time,
        url: url.to_string(),
        status,
        content_length,
        downloaded_size: downloaded,
        file_path: filename,
    })
}
