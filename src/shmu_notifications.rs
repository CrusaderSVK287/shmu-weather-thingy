#[cfg(any(target_os = "windows"))]
use std::path::Path;
use std::path::PathBuf;

#[cfg(all(unix, not(target_os = "macos")))] 
use notify_rust::{Notification}; 
#[cfg(any(target_os = "windows"))] 
use winrt_notification::{Duration, IconCrop, Sound, Toast};

pub struct SHMUNotification {
    headline: String,
    body: String,
    icon: String,
}

// Methods common for both windows and linux
impl SHMUNotification {
    pub fn new(headline: &str, body: &str) -> Self{
        Self {
            headline: String::from(headline), 
            body: String::from(body),
            icon: String::new()
        }
    }

    #[allow(dead_code)]
    pub fn with_icon(headline: &str, body: &str, icon: &str) -> Self{
        Self {
            headline: String::from(headline), 
            body: String::from(body),
            icon: String::from(icon)
        }
    }

    pub fn send(&self) {
        let res = self.display_notification();

        match res {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to display notification: {:?}", e);
            }
        }
    }
}

fn notification_icon_path() -> PathBuf {
    let path = std::env::temp_dir().join("shmu-weather-wind.png");

    // Only write the file once.
    if !path.exists() {
        std::fs::write(&path, WIND_ICON)
            .expect("failed to write embedded notification icon");
    }

    path
}

static WIND_ICON: &[u8] = include_bytes!("../assets/icons/wind.png");
// Methods that are OS specific for linux
#[cfg(all(unix, not(target_os = "macos")))]
impl SHMUNotification {
    fn display_notification(&self) -> Result<(), notify_rust::error::Error> {
        let icon = format!("file://{}", notification_icon_path().display());

        Notification::new()
            .summary(&self.headline)
            .body(&self.body)
            .icon(&icon)
            .show()?;

        Ok(())
    }
}

// Methods that are OS specific for windows
#[cfg(target_os = "windows")]
impl SHMUNotification {
    fn display_notification(&self) -> Result<(), winrt_notification::Error>{
        let res = Toast::new(Toast::POWERSHELL_APP_ID)
            .title(self.headline.as_str())
            .text1(self.body.as_str())
            .icon(Path::new(&self.icon), IconCrop::Square, "")
            .sound(Some(Sound::SMS))
            .duration(Duration::Short)
            .show();
        res
    }
}
