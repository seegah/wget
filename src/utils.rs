use std::fs;
use std::path::PathBuf;
use chrono::Local;
use url::Url;

pub fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_size(size: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as usize, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

pub fn get_filename_from_url(url: &str) -> PathBuf {
    let parsed = Url::parse(url).unwrap_or_else(|_| {
        // If URL parsing fails, try to extract the last part of the path
        let parts: Vec<&str> = url.split('/').collect();
        let filename = parts.last().unwrap_or(&"index.html");
        Url::parse(&format!("http://localhost/{}", filename)).unwrap()
    });

    PathBuf::from(parsed.path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("index.html")
        .to_string()
        .split('?')
        .next()
        .unwrap_or("index.html"))
}

pub fn read_urls_from_file(file: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    Ok(fs::read_to_string(file)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_get_filename_from_url() {
        assert_eq!(
            get_filename_from_url("https://example.com/file.txt"),
            PathBuf::from("file.txt")
        );
        assert_eq!(
            get_filename_from_url("https://example.com/"),
            PathBuf::from("index.html")
        );
        assert_eq!(
            get_filename_from_url("https://example.com/file.txt?param=value"),
            PathBuf::from("file.txt")
        );
    }
} 