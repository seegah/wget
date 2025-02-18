use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// URL to download
    pub url: Option<String>,

    /// Input file containing URLs (one per line)
    #[arg(short = 'i', long, value_name = "FILE")]
    pub input_file: Option<PathBuf>,

    /// Output file name
    #[arg(short = 'O', long)]
    pub output: Option<PathBuf>,

    /// Directory prefix
    #[arg(short = 'P', long, value_name = "DIR")]
    pub directory_prefix: Option<PathBuf>,

    /// Download rate limit (e.g., "300k", "2M")
    #[arg(long = "rate-limit")]
    pub rate_limit: Option<String>,

    /// Mirror entire website
    #[arg(long)]
    pub mirror: bool,

    /// Convert links for offline viewing
    #[arg(long = "convert-links")]
    pub convert_links: bool,

    /// Background mode
    #[arg(short = 'B', long)]
    pub background: bool,

    /// Reject file patterns
    #[arg(long = "reject")]
    pub reject: Option<String>,

    /// Exclude directories
    #[arg(short = 'X', long = "exclude-directories")]
    pub exclude_directories: Option<String>,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

impl CliArgs {
    pub fn get_output_path(&self, url: &str) -> PathBuf {
        let filename = if let Some(ref output) = self.output {
            output.clone()
        } else {
            crate::utils::get_filename_from_url(url)
        };

        if let Some(ref prefix) = self.directory_prefix {
            prefix.join(filename)
        } else {
            filename
        }
    }

    pub fn parse_rate_limit(&self) -> Option<u64> {
        self.rate_limit.as_ref().and_then(|limit| {
            let limit = limit.to_lowercase();
            if limit.ends_with('k') {
                limit[..limit.len()-1].parse::<u64>().ok().map(|n| n * 1024)
            } else if limit.ends_with('m') {
                limit[..limit.len()-1].parse::<u64>().ok().map(|n| n * 1024 * 1024)
            } else {
                limit.parse::<u64>().ok()
            }
        })
    }
}