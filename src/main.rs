use scraper::{Html, Selector};

use xml::reader::{EventReader, XmlEvent};

const BASE_URL: &str =
    "http://opendata.shmu.sk/meteorology/weather/alerts/cap/";

struct Alert {
    area_desc: String,
    event: String,
    headline: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // 1. base directory
    let html = shmu_get_html(BASE_URL).await?;

    let day = extract_last_folder(&html)
        .expect("No day folder found");

    let day_url = format!("{BASE_URL}{day}/");
    println!("Day: {day_url}");

    // 2. timestamp directory
    let day_html = shmu_get_html(&day_url).await?;

    let timestamp = extract_last_folder(&day_html)
        .expect("No timestamp folder found");

    let ts_url = format!("{day_url}{timestamp}/");
    println!("Timestamp: {ts_url}");

    // 3. fetch XML directory
    let files_html = shmu_get_html(&ts_url).await?;

    let files = extract_xml_files(&files_html);

    // 4. loop through all XML files
    for file in files {
        let file_url = format!("{ts_url}{file}");
        println!("Fetching: {file_url}");

        let xml = shmu_get_html(&file_url).await?;

        handle_xml(&xml).await;
        // TODO: Put away this break, its just to not dos the opendata.shmu.sk server lol
        break;
    }

    Ok(())
}

// ---------------- HTTP ----------------

async fn shmu_get_html(url: &str) -> Result<String, reqwest::Error> {
    reqwest::get(url).await?.text().await
}

// ---------------- PARSING ----------------

fn extract_last_folder(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();

    document
        .select(&selector)
        .filter_map(|e| e.value().attr("href"))
        .filter_map(|href| href.strip_suffix('/'))
        .filter(|s| s.chars().all(|c| c.is_ascii_digit()))
        .last()
        .map(|s| s.to_string())
}

fn extract_xml_files(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();

    document
        .select(&selector)
        .filter_map(|e| e.value().attr("href"))
        .filter(|href| href.ends_with(".xml"))
        .map(|s| s.to_string())
        .collect()
}

// ---------------- HANDLER ----------------

async fn handle_xml(xml: &str) {
    let mut current_element: Option<String> = None;

    let mut alert = Alert {
        area_desc: String::new(),
        event: String::new(),
        headline: String::new(),
        description: String::new(),
    };

    let parser = EventReader::from_str(xml);

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                current_element = Some(name.local_name);
            }

            Ok(XmlEvent::Characters(text)) => {
                let text = text.trim();
                if text.is_empty() {
                    continue;
                }

                match current_element.as_deref() {
                    Some("areaDesc") if alert.area_desc.is_empty() => {
                        alert.area_desc.push_str(text)
                    }
                    Some("event") if alert.event.is_empty() => {
                        alert.event.push_str(text)
                    }
                    Some("headline") if alert.headline.is_empty() => {
                        alert.headline.push_str(text)
                    }
                    Some("description") if alert.description.is_empty() => {
                        alert.description.push_str(text)
                    }
                    _ => {}
                }
            }

            Ok(XmlEvent::EndElement { .. }) => {
                current_element = None;
            }

            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }

            _ => {}
        }
    }

    process_area(&alert);
}

fn process_area(alert: &Alert) {
    if alert.area_desc != "Dolný Kubín" {
        return;
    }

    println!("Area: {}", alert.area_desc);
    println!("Event: {}", alert.event);
    println!("Headline: {}", alert.headline);
    //println!("Description: {}", alert.description);
}
