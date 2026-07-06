use serde::Deserialize;

use crate::{shmu_config::Config, shmu_notifications::SHMUNotification};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    #[default]
    Mild,
    Moderate,
    Severe,
    Extreme,
    Unknown
}

impl std::str::FromStr for AlertSeverity {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str()  {
            "mild" => Ok(AlertSeverity::Mild),
            "moderate" => Ok(AlertSeverity::Moderate),
            "severe" => Ok(AlertSeverity::Severe),
            "extreme" => Ok(AlertSeverity::Extreme),
            "unknown" => Ok(AlertSeverity::Unknown),
            _ => Err(format!("Unspecified severity: {}", input)),        }
    }
}

#[derive(Debug)]
pub struct Alert {
    pub area_desc: String,
    pub event: String,
    pub headline: String,
    pub description: String,
    pub severity: AlertSeverity,
}

impl Alert {
    pub fn empty() -> Self {
        Self {
            area_desc: String::new(),
            event: String::new(),
            headline: String::new(),
            description: String::new(),
            severity: AlertSeverity::Unknown
        }
    }

    pub fn process(&self, cfg: &Config) {
        let mut headline = String::new();
        headline.push_str(&self.area_desc);
        headline.push_str(": ");
        headline.push_str(&self.event);

        let mut body = String::new();
        body.push_str(&self.headline);
        if cfg.include_description {
            body.push_str("\n\n");
            body.push_str(&self.description);
        }

        if cfg.notifications {
            SHMUNotification::new(&headline, &body).send();
        }
    }

    pub fn should_handle(&self, cfg: &Config) -> bool {
        // If severity not big enough, dont handle
        if self.severity < cfg.min_severity {
            return false
        }
        // If about area we dont care about, dont handle
        if !cfg.includes_area(&self.area_desc) {
            return false
        }
        true
    }
}