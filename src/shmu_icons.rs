use std::fs;
use std::io;
use crate::shmu_alert::{AlertType};
use directories::ProjectDirs;

static WIND_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/wind.png");

pub fn icon_path(alert_type: AlertType) -> String {
    let proj_dirs = ProjectDirs::from(
        "com",
        "CrusaderSVK287",
        "SHMU_CAP_Weather_Alert_Monitor",
    )
    .ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory",
        )
    });

    let binding = proj_dirs.unwrap();
    let config_dir = binding.config_dir();
    let icons_dir = config_dir.join("/icons");
    let icon_path = icons_dir.join("/wind.png");

    let _ = fs::create_dir_all(config_dir);

    if !icon_path.exists() {
        create_default_icons(String::from(icons_dir));
    }

    String::new()
}

fn create_default_icons(base_path: String) {

}

/*
fn notification_icon_path() -> PathBuf {
    let path = std::env::temp_dir().join("shmu-weather-wind.png");

    // Only write the file once.
    if !path.exists() {
        std::fs::write(&path, WIND_ICON)
            .expect("failed to write embedded notification icon");
    }

    path
}
*/