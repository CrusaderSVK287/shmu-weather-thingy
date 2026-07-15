mod shmu_fetch;
mod shmu_notifications;
mod shmu_config;
mod shmu_alert;
mod shmu_default_config;
mod shmu_icons;
mod shmu_db;

use env_logger::Env;
use tokio::time::{sleep, Duration};
use log::{error, info};

use crate::{shmu_config::Config, shmu_db::AlertDatabase, shmu_fetch::SHMUClient};

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

    let db = match if cfg.persistent {
        AlertDatabase::new(&cfg.db_path)
    } else {
        AlertDatabase::empty()
    } {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }
    };

    if cfg.persistent {
        let _ = db.clear_expired_alerts();
    }

    loop {
        if let Err(err) = shmu_client.run_alert_scan(&cfg, &db).await {
            error!("Error: {}", err);
        }

        if cfg.run_once {
            break
        }

        sleep(Duration::from_secs(cfg.period())).await;
    }
}
