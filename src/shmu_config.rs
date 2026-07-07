use serde::Deserialize;
use toml;
use std::fs;
use log::{error};
use crate::shmu_alert::{AlertSeverity, AlertType};

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    // List of areas to handle
    area: Vec<String>,
    // level of logging
    pub log_level: String,
    // Period stating how often alerts should be pulled. In minutes
    #[serde(default = "default_period")]
    period: u64,
    // Includes the long description in the notification body
    pub include_description: bool,
    // whether desltop notifications will be used or not, usefull for headless servers or silent running
    pub notifications: bool,
    // Alert severity from which that and more sever alerts should be shown
    pub min_severity: AlertSeverity,
    // List of alert types (numerical id, check shmu_alerts.rs) to be excluded from being processed.
    // If empty, all alert types are processed regardless of alert_types_is_allowlist
    alert_types: Vec<u8>,
    // if false, alert_types filters out unwanted alerts, if true, only listed alerts are handled
    alert_types_is_allowlist: bool,

    // Debug configurations
    // Fetch and handle only one alert, used in development to not overwhelm the shmu server
    pub _fetch_only_one_alert: bool,
    // Prints alert data before sending notification
    pub _print_alert_before_sending_notification: bool,
}

impl Config {
    pub fn new() -> Result<Self, std::io::Error> {
        // TODO: Add absolute path based on OS probably
        let tom_text: String = fs::read_to_string("src/config.toml")?;

        let config: Config = toml::from_str(tom_text.as_str()).unwrap();
        Ok(config)
    }

    // serde default
    #[allow(dead_code)]
    fn default() -> Self {
        Self {
            area: Vec::new(),
            log_level: "error".to_string(),
            period: 60,
            include_description: false,
            notifications: true,
            min_severity: AlertSeverity::Mild, // this means all alerts will be shown
            alert_types: Vec::new(),
            alert_types_is_allowlist: false,

            _fetch_only_one_alert: false,
            _print_alert_before_sending_notification: false,
        }
    }

    pub fn _print(&self) {
        println!("{:#?}", self);
    }

    pub fn period(&self) -> u64 {
        // The period is in minutes, hence why we multiply by 60
        self.period * 60
    }

    pub fn includes_area(&self, area: &str) -> bool {
        self.area.contains(&area.to_string())
    }

    pub fn should_handle_alert_type(&self, alert: AlertType) -> bool {
        // An empty filter means no restrictions: handle all alert types.
        if self.alert_types.is_empty() {
            return true;
        }

        let is_listed = self.alert_types.contains(&(alert as u8));

        if self.alert_types_is_allowlist {
            // Allowlist: only handle alert types explicitly included in the list.
            is_listed
        } else {
            // Exclude list: handle all alert types except those in the list.
            !is_listed
        }
    }

    pub fn verify_config(&self) -> bool {
        // Empty area means no alerts will be reported
        if self.area.is_empty() {
            error!("No area specified in the configuration, no alert would be handled");
            return false;
        }
        if self.min_severity == AlertSeverity::Unknown {
            error!("min_severity is specified as Unknown, fix the configuration");
            return false;
        }

        true
    }
}

fn default_period() -> u64 {
    60
}
