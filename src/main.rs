mod cli;
mod downloader;

use crate::cli::cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  cli()
}
