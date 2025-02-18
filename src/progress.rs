use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct ProgressTracker {
    progress_bar: ProgressBar,
}

impl ProgressTracker {
    pub fn new(total_size: u64) -> Self {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {bytes:>8}/{total_bytes:>8} ({percent:>3}%) ETA: {eta}")
                .unwrap()
                .progress_chars("#>-")
        );
        
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_message("Downloading:");
        
        Self {
            progress_bar: pb,
        }
    }

    pub fn update(&self, progress: u64) {
        self.progress_bar.set_position(progress);
    }

    pub fn finish(&self) {
        self.progress_bar.finish_with_message("Download complete");
    }
}