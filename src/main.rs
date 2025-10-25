use clap::{App, Arg, SubCommand};
use reqwest;
use scraper::{Html, Selector};
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let matches = App::new("sppd-cli")
    .version("0.1")
    .author("Alvaro Carranza <alvarocarranzacarrion@gmail.com>")
    .about("Downloads and parses Spanish Public Procurement Data in Rust")
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

    // Rest of your existing code here
    let input_url = "http://example.com"; // Placeholder for the input URL
    println!("Input type: {}", input_type);

    let base_url = match Url::parse(input_url) {
      Ok(url) => url,
      Err(_) => {
        eprintln!("Invalid URL: {}", input_url);
        std::process::exit(1);
      }
    };

    let response = reqwest::blocking::get(base_url.clone())?.text()?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse("a").unwrap();

    let mut links = std::collections::HashSet::new();
    for element in document.select(&selector) {
      if let Some(link) = element.value().attr("href") {
        let absolute_link = base_url.join(link).unwrap();
        links.insert(absolute_link.to_string());
      }
    }

    for link in links {
      println!("{}", link);
    }
  }

  Ok(())
}
