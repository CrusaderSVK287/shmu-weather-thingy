mod shmu_fetch;
mod shmu_notifications;
mod shmu_config;
mod shmu_alert;
mod shmu_default_config;
mod shmu_icons;

use env_logger::Env;
use tokio::time::{sleep, Duration};
use log::{error, info};

use crate::{shmu_config::Config, shmu_fetch::SHMUClient};

#[tokio::main]
async fn main() {
    let cfg = match Config::new() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            return;
        }
    };
    //cfg._print();
    let mut shmu_client = SHMUClient::new();
    
    env_logger::Builder::from_env(Env::default().default_filter_or(&cfg.log_level)).init();    
    info!("Logger initialized");

    // Since logger is only initialized after configuration is created, we need to verify it here.
    if !cfg.verify_config() {
        return;
    }

    loop {
        if let Err(err) = shmu_client.run_alert_scan(&cfg).await {
            error!("Error: {}", err);
        }

        sleep(Duration::from_secs(cfg.period())).await;
    }
}
