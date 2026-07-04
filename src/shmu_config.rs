use serde::Deserialize;
use toml;
use std::fs;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    area: Vec<String>,
    log_level: String,
    #[serde(default = "default_period")]
    period: u64,
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
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.area);
        println!("{}", self.log_level);
        println!("{}", self.period);
    }

    pub fn log_level(&self) -> &str {
        self.log_level.as_str()
    }
    pub fn period(&self) -> u64 {
        // The period is in minutes, hence why we multiply by 60
        self.period * 60
    }

    pub fn includes_area(&self, area: &str) -> bool {
        self.area.contains(&area.to_string())
    }
}

fn default_period() -> u64 {
    60
}
