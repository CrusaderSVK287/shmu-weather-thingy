use scraper::{Html, Selector};
use xml::reader::{EventReader, XmlEvent};
use log::{error, info};
use crate::shmu_config::{Config};
use crate::shmu_alert::{Alert};

pub struct SHMUClient {
    last_fetched_url: String
}

const BASE_URL: &str =
    "http://opendata.shmu.sk/meteorology/weather/alerts/cap/";


impl SHMUClient {
    pub fn new() -> Self {
        Self {
            last_fetched_url: String::from(""),
        }
    }

    pub async fn run_alert_scan(&mut self, cfg: &Config) -> Result<(), reqwest::Error> {
        // 1. base directory
        let html = Self::get_html(BASE_URL).await?;
        let day = match Self::extract_last_folder(&html) {
            Some(d) => d,
            None => {
                error!("No day folder found");
                return Ok(());
            }
        };
        let day_url = format!("{BASE_URL}{day}/");

        // 2. timestamp directory
        let day_html = Self::get_html(&day_url).await?;

        let timestamp = match Self::extract_last_folder(&day_html) {
            Some(t) => t,
            None => {
                error!("No timestamp folder found");
                return Ok(());
            }
        };
        let ts_url = format!("{day_url}{timestamp}/");

        // 2.1 check if the timestamp folder is the same as last fetched folder, if yes, cancel
        if ts_url == self.last_fetched_url {
            info!("No new data found, fetching canceled");
            return Ok(())
        }
        self.last_fetched_url = ts_url.clone();
        info!("Timestamp: {ts_url}");

        // 3. fetch XML directory
        let files_html = Self::get_html(&ts_url).await?;
        let files = Self::extract_xml_files(&files_html);

        // 4. loop through all XML files
        for file in files {
            let file_url = format!("{ts_url}{file}");
            info!("Fetching: {file_url}");
            let xml = Self::get_html(&file_url).await?;
            let alert = Self::generate_alert_from_xml(&xml);
            if cfg.includes_area(&alert.area_desc) {
                alert.process(cfg);
            }

            // For debugging purposes, only fetch the first CAP, not all of them
            if cfg._fetch_only_one_alert {
                println!("DEBUG: Area of the first fetched alert: {a}", a = alert.area_desc);
                break;
            }
        }

        info!("Finished scan of SHMU CAP alerts");
        Ok(())
    }

    async fn get_html(url: &str) -> Result<String, reqwest::Error> {
        reqwest::get(url).await?.text().await
    }

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

    fn generate_alert_from_xml(xml: &str) -> Alert {
        let mut current_element: Option<String> = None;

        let mut alert = Alert::empty();

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
                    error!("Error: {e}");
                    break;
                }

                _ => {}
            }
        }

        alert
    }
}
