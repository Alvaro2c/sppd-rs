use clap::{Command, Arg, ArgAction};
use std::collections::HashMap;
use crate::models::ProcurementType;

pub fn cli(
  minor_contracts_links: HashMap<String, String>,
  public_tenders_links: HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
  let matches = Command::new("sppd-cli")
    .version("0.1")
    .author("Alvaro Carranza <alvarocarranzacarrion@gmail.com>")
    .about("Downloads and parses Spanish Public Procurement Data (SPPD)")
    .subcommand(
      Command::new("download")
        .about("Download procurement data")
        .arg(
          Arg::new("type")
            .short('t')
            .long("type")
            .help("Procurement type: 'minor-contracts' (mc, min) or 'public-tenders' (pt, pub)")
            .default_value("public-tenders")
            .action(ArgAction::Set),
        ),
    )
    .get_matches();

  if let Some(download_matches) = matches.subcommand_matches("download") {
    let proc_type = ProcurementType::from(download_matches.get_one::<String>("type").unwrap().as_str());

    let links = match proc_type {
      ProcurementType::MinorContracts => &minor_contracts_links,
      ProcurementType::PublicTenders => &public_tenders_links,
    };

    for (period, url) in links {
      println!("Period: {}, URL: {}", period, url);
    }
  }

  Ok(())
}
