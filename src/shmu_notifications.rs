#[cfg(any(target_os = "windows"))]
use std::path::Path;
use std::path::PathBuf;
use crate::shmu_icons;
#[cfg(all(unix, not(target_os = "macos")))]
use crate::{shmu_alert::AlertType, shmu_icons::icon_path};

#[cfg(all(unix, not(target_os = "macos")))] 
use notify_rust::{Notification}; 
#[cfg(any(target_os = "windows"))] 
use winrt_notification::{Duration, IconCrop, Sound, Toast};

pub struct SHMUNotification {
    headline: String,
    body: String,
}

// Methods common for both windows and linux
impl SHMUNotification {
    pub fn new(headline: &str, body: &str) -> Self{
        Self {
            headline: String::from(headline), 
            body: String::from(body)
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

// Methods that are OS specific for linux
#[cfg(all(unix, not(target_os = "macos")))]
impl SHMUNotification {
    fn display_notification(&self) -> Result<(), notify_rust::error::Error> {
        Notification::new()
            .summary(&self.headline)
            .body(&self.body)
            .icon(icon_path(AlertType::Wind).as_str())
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
            .icon(Path::new(icon_path(AlertType::Wind).as_str()), IconCrop::Square, "")
            .sound(Some(Sound::SMS))
            .duration(Duration::Short)
            .show();
        res
    }
}
