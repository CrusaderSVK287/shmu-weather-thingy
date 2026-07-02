#[cfg(all(unix, not(target_os = "macos")))] 
use notify_rust::{Notification}; 
#[cfg(any(target_os = "windows"))] 
use winrt_notification::{Duration, Sound, Toast};

pub struct SHMUNotification {
    headline: String,
    body: String,
    icon: String,
}

// Methods common for both windows and linux
impl SHMUNotification {
    pub fn new(headline: String, body: String, icon: String) -> Self{
        Self {
            headline, body, icon
        }
    }
}

// Methods that are OS specific for linux
#[cfg(all(unix, not(target_os = "macos")))]
impl SHMUNotification {
    pub fn send(&self) {
        let _ = Notification::new()
        .summary(self.headline.as_str())
        .body(self.body.as_str())
        .icon(self.icon.as_str())
        .show();
    }
}

// Methods that are OS specific for windows
#[cfg(any(target_os = "windows"))]
impl SHMUNotification {
    pub fn send(&self) {
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(self.headline)
            .text1(self.body)
            .sound(Some(Sound::SMS))
            .duration(Duration::Short)
            .show()
            .expect("unable to toast");
    }
}