use log::info;
use serde::{Deserialize, Serialize};

use crate::{shmu_config::Config, shmu_notifications::SHMUNotification};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Deserialize, Default, Serialize)]
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AlertType {
    Wind = 1,
    SnowIce = 2,
    Thunderstorm = 3,
    Fog = 4,
    HighTemperature = 5,
    LowTemperature = 6,
    CoastalEvent = 7,
    ForestFire = 8,
    Avalanches = 9,
    Rain = 10,
    #[default]
    Unknown = 11, // Legacy value
    Flooding = 12,
    RainFlood = 13,
    MarineHazard = 14,
    Drought = 15,
}

impl TryFrom<u8> for AlertType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Wind),
            2 => Ok(Self::SnowIce),
            3 => Ok(Self::Thunderstorm),
            4 => Ok(Self::Fog),
            5 => Ok(Self::HighTemperature),
            6 => Ok(Self::LowTemperature),
            7 => Ok(Self::CoastalEvent),
            8 => Ok(Self::ForestFire),
            9 => Ok(Self::Avalanches),
            10 => Ok(Self::Rain),
            11 => Ok(Self::Unknown),
            12 => Ok(Self::Flooding),
            13 => Ok(Self::RainFlood),
            14 => Ok(Self::MarineHazard),
            15 => Ok(Self::Drought),
            _ => Err(String::from("Failed to convert u8 to AlertType")),
        }
    }
}

impl From<AlertType> for u8 {
    fn from(value: AlertType) -> Self {
        value as u8
    }
}

#[derive(Debug)]
pub struct Alert {
    pub area_desc: String,
    pub event: String,
    pub headline: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub alert_type: AlertType
}

impl Alert {
    pub fn empty() -> Self {
        Self {
            area_desc: String::new(),
            event: String::new(),
            headline: String::new(),
            description: String::new(),
            severity: AlertSeverity::Unknown,
            alert_type: AlertType::Unknown,
        }
    }

    pub fn process(&self, cfg: &Config) {
        if cfg._print_alert_before_sending_notification {
            println!("{:#?}", self);
        }

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

        info!("Alert processed, sending notification if enabled");
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
        // Alert type filtering
        if !cfg.should_handle_alert_type(self.alert_type) {
            return false
        }

        true
    }
}