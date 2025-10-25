use clap::{App, Arg, SubCommand};
use reqwest;
use scraper::{Html, Selector};
use url::Url;
use regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  cli()
}

fn cli() -> Result<(), Box<dyn std::error::Error>> {
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
        .help("Sets the type of the request: 'small' (or 's') or 'regular' (or 'r')")))
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

    // determine the input URL based on the type
    println!("Input type: {}", input_type);
    let input_url = match input_type {
      "small" => "https://www.hacienda.gob.es/es-es/gobiernoabierto/datos%20abiertos/paginas/contratosmenores.aspx",
      "regular" => "https://www.hacienda.gob.es/es-ES/GobiernoAbierto/Datos%20Abiertos/Paginas/LicitacionesContratante.aspx",
      _ => {
        eprintln!("Unknown input type: {}", input_type);
        std::process::exit(1);
      }
    };

    // fetch and print links
    let links = fetch_links(input_url)?;
    for (period, url) in links {
      println!("Period: {}, URL: {}", period, url);
    }
  }

  Ok(())
}


fn fetch_links(input_url: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
  // parse the base URL
  let base_url = Url::parse(input_url)?;

  // fetch the page content
  let response = reqwest::blocking::get(base_url.as_str())?
    .error_for_status()?
    .text()?;
  let document = Html::parse_document(&response);

  // selector to find all links ending with .zip
  let selector = Selector::parse(r#"a[href$=".zip"]"#).unwrap();

  let mut links: std::collections::HashMap<String, String> = std::collections::HashMap::new();
  let re = regex::Regex::new(r"_(\d+)\.zip$")?;
  for url in document.select(&selector)
    .filter_map(|el| el.value().attr("href"))
    .filter_map(|href| base_url.join(href).ok())
  {
    if let Some(filename) = url.path_segments().and_then(|s| s.last()) {
      if let Some(m) = re.captures(filename).and_then(|c| c.get(1)) {
        links.insert(m.as_str().to_string(), url.to_string());
      }
    }
  }

  Ok(links)
}
