mod shmu_fetch;
use std::{thread, time};

use crate::shmu_fetch::SHMUClient;

#[tokio::main]
async fn main() {
    let shmu_client = SHMUClient::new(10);

    loop {
        if let Err(err) = shmu_client.shmu_run_alert_scan().await {
            eprintln!("Error: {}", err);
        }

        thread::sleep(time::Duration::from_secs(shmu_client.scan_period_secs));
    }
}
