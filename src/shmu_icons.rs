use std::fs;
use std::io;
use std::path::Path;
use crate::shmu_alert::{AlertType};
use directories::ProjectDirs;
use log::{warn};

static WIND_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/wind.png");
static SNOW_ICE_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/snow_ice.png");
static THUNDERSTORM_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/thunderstorm.png");
static FOG_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/fog.png");
static HIGH_TEMPERATURE_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/high_temperature.png");
static LOW_TEMPERATURE_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/low_temperature.png");
static COASTAL_EVENT_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/coastal_event.png");
static FOREST_FIRE_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/forest_fire.png");
static AVALANCHES_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/avalanches.png");
static RAIN_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/rain.png");
static UNKNOWN_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/unknown.png");
static FLOODING_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/flooding.png");
static RAIN_FLOOD_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/rain_flood.png");
static MARINE_HAZARD_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/marine_hazard.png");
static DROUGHT_ICON_BYTES: &[u8] = include_bytes!("../assets/icons/drought.png");

const DEFAULT_ICONS: &[(&str, &[u8])] = &[
    ("wind.png", WIND_ICON_BYTES),
    ("snow_ice.png", SNOW_ICE_ICON_BYTES),
    ("thunderstorm.png", THUNDERSTORM_ICON_BYTES),
    ("fog.png", FOG_ICON_BYTES),
    ("high_temperature.png", HIGH_TEMPERATURE_ICON_BYTES),
    ("low_temperature.png", LOW_TEMPERATURE_ICON_BYTES),
    ("coastal_event.png", COASTAL_EVENT_ICON_BYTES),
    ("forest_fire.png", FOREST_FIRE_ICON_BYTES),
    ("avalanches.png", AVALANCHES_ICON_BYTES),
    ("rain.png", RAIN_ICON_BYTES),
    ("unknown.png", UNKNOWN_ICON_BYTES),
    ("flooding.png", FLOODING_ICON_BYTES),
    ("rain_flood.png", RAIN_FLOOD_ICON_BYTES),
    ("marine_hazard.png", MARINE_HAZARD_ICON_BYTES),
    ("drought.png", DROUGHT_ICON_BYTES),
];

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
    let icons_dir = config_dir.join("icons");

    let icon_path = match alert_type {
        AlertType::Wind => icons_dir.join("wind.png"),
        AlertType::SnowIce => icons_dir.join("snow_ice.png"),
        AlertType::Thunderstorm => icons_dir.join("thunderstorm.png"),
        AlertType::Fog => icons_dir.join("fog.png"),
        AlertType::HighTemperature => icons_dir.join("high_temperature.png"),
        AlertType::LowTemperature => icons_dir.join("low_temperature.png"),
        AlertType::CoastalEvent => icons_dir.join("coastal_event.png"),
        AlertType::ForestFire => icons_dir.join("forest_fire.png"),
        AlertType::Avalanches => icons_dir.join("avalanches.png"),
        AlertType::Rain => icons_dir.join("rain.png"),
        AlertType::Unknown => icons_dir.join("unknown.png"),
        AlertType::Flooding => icons_dir.join("flooding.png"),
        AlertType::RainFlood => icons_dir.join("rain_flood.png"),
        AlertType::MarineHazard => icons_dir.join("marine_hazard.png"),
        AlertType::Drought => icons_dir.join("drought.png"),
    };

    let _ = fs::create_dir_all(&icons_dir);
    if !icon_path.exists() {
        match create_default_icons(icons_dir.as_os_str().to_str().unwrap()) {
            Ok(_) => (),
            Err(e) => {
                warn!("Failed to create default icons: {e}");
            },
        }
    }

    String::from(icon_path.to_str().unwrap())
}

fn create_default_icons(base_path: &str) -> Result<(), io::Error> {
    let path = Path::new(base_path);

    for (name, bytes) in DEFAULT_ICONS {
        fs::write(path.join(name), bytes)?;
    }

    Ok(())
}