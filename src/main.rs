mod shmu_fetch;
mod shmu_notifications;
use env_logger::Env;
use tokio::time::{sleep, Duration};
use log::{error, info};

use crate::{shmu_fetch::SHMUClient, shmu_notifications::SHMUNotification};

#[tokio::main]
async fn main() {
    let mut shmu_client = SHMUClient::new(10);
    
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();    info!("SHMU weather application started");
    SHMUNotification::new(
    "SHMU weather",
        "Application started"
    ).send();

    loop {
        if let Err(err) = shmu_client.run_alert_scan().await {
            error!("Error: {}", err);
        }

        sleep(Duration::from_secs(shmu_client.scan_period())).await;
    }
}
