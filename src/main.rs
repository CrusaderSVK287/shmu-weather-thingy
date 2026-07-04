mod shmu_fetch;
mod shmu_notifications;
mod shmu_config;

use env_logger::Env;
use tokio::time::{sleep, Duration};
use log::{error, info};

use crate::{shmu_config::Config, shmu_fetch::SHMUClient, shmu_notifications::SHMUNotification};

#[tokio::main]
async fn main() {
    let cfg = match Config::new() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load config: {}", err);
            return;
        }
    };
    cfg.print();
    let mut shmu_client = SHMUClient::new();
    
    env_logger::Builder::from_env(Env::default().default_filter_or(cfg.log_level())).init();    
    info!("SHMU weather application started");
    SHMUNotification::new("SHMU weather","Application started").send();

    loop {
        if let Err(err) = shmu_client.run_alert_scan(&cfg).await {
            error!("Error: {}", err);
        }

        sleep(Duration::from_secs(cfg.period())).await;
    }
}
