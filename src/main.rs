mod cli;
mod downloader;
mod mirror;
mod progress;
mod utils;

fn main() {
    let args = cli::parse_args();
    
    if args.mirror {
        let _ = mirror::mirror_site(args);
    } else {
        let _ = downloader::download(args);
    }
}