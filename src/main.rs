mod shmu_fetch;
mod shmu_notifications;
use tokio::time::{sleep, Duration};


use crate::{shmu_fetch::SHMUClient, shmu_notifications::SHMUNotification};

#[tokio::main]
async fn main() {
    let mut shmu_client = SHMUClient::new(10);

    let notif = SHMUNotification::new(
    String::from("SHMU weather"),
        String::from("Application started"), 
        String::from("")
    );
    notif.send();

    loop {
        if let Err(err) = shmu_client.shmu_run_alert_scan().await {
            eprintln!("Error: {}", err);
        }

        sleep(Duration::from_secs(shmu_client.scan_period_secs)).await;
    }
}
