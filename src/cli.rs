use clap::{App, Arg, SubCommand};
use std::collections::HashMap;

pub fn cli(
  minor_contracts_links: HashMap<String, String>,
  public_tenders_links: HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
  let matches = App::new("sppd-cli")
    .version("0.1")
    .author("Alvaro Carranza <alvarocarranzacarrion@gmail.com>")
    .about("Downloads and parses Spanish Public Procurement Data (SPPD))")
    .subcommand(SubCommand::with_name("download")
      .about("Download procurement data")
      .arg(Arg::new("type")
        .short('t')
        .long("type")
        .takes_value(true)
        .help("Sets the type of the procurement: 'minor-contracts' (or 'c') or 'pubic-tenders' (or 't')")))
    .get_matches();

  if let Some(download_matches) = matches.subcommand_matches("download") {
    let raw = download_matches.value_of("type").unwrap_or("regular").to_lowercase();
    let input_type = match raw.as_str() {
      "s" | "small" => "small",
      "r" | "regular" => "regular",
      _ => {
        eprintln!("Invalid type. Use 'small' (or 's') or 'regular' (or 'r').");
        return Ok(());
      }
    };

    // determine the input links based on the type
    println!("Input type: {}", input_type);
    let links = match input_type {
      "small" => minor_contracts_links,
      "regular" => public_tenders_links,
      _ => {
        eprintln!("Unknown input type: {}", input_type);
        std::process::exit(1);
      }
    };

    // print links
    for (period, url) in links {
      println!("Period: {}, URL: {}", period, url);
    }
  }

  Ok(())
}
