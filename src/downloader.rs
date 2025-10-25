use reqwest;
use scraper::{Html, Selector};
use url::Url;
use regex;

pub fn fetch_links(input_url: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
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
