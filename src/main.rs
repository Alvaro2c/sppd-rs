mod config;
mod cli;
mod downloader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let (
    minor_contracts_links,
    public_tenders_links
  ) = downloader::fetch_all_links()?;
  cli::cli(minor_contracts_links, public_tenders_links)?;
  Ok(())
}
