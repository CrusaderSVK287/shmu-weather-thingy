use ntfy::prelude::*;

#[cfg(any(target_os = "windows"))]
use std::path::Path;
use crate::{shmu_alert::AlertType, shmu_config::Config, shmu_icons::icon_path};

#[cfg(all(unix, not(target_os = "macos")))] 
use notify_rust::{Notification}; 
#[cfg(any(target_os = "windows"))] 
use winrt_notification::{Duration, IconCrop, Sound, Toast};

pub struct SHMUNotification {
    headline: String,
    body: String,
    icon: AlertType
}

// Methods common for both windows and linux
impl SHMUNotification {
    #[allow(dead_code)]
    pub fn new(headline: &str, body: &str, icon: AlertType) -> Self{
        Self {
            headline: String::from(headline), 
            body: String::from(body),
            icon: icon
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

    pub fn send_push(cfg: &Config, headline: &str, body: &str) -> Result<(), Error> {
        let dispatcher = dispatcher::builder("https://ntfy.sh").build_blocking()?; // Build dispatcher

        let payload = Payload::new(cfg.ntfy_topic.as_str())
            .message(String::from(body))
            .title(String::from(headline))
            .tags(["warning"]) 
            .priority(Priority::Default)
            .click(Url::parse("http://www.shmu.sk/?page=987")?)
            .markdown(true);

        dispatcher.send(&payload)?;

        Ok(())
    }
}

// Methods that are OS specific for linux
#[cfg(all(unix, not(target_os = "macos")))]
impl SHMUNotification {
    fn display_notification(&self) -> Result<(), notify_rust::error::Error> {
        Notification::new()
            .summary(&self.headline)
            .body(&self.body)
            .icon(icon_path(self.icon).as_str())
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
            .icon(Path::new(icon_path(self.icon).as_str()), IconCrop::Square, "")
            .sound(Some(Sound::Reminder))
            .duration(Duration::Short)
            .show();
        res
    }
}
