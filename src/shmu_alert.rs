use crate::{shmu_config::Config, shmu_notifications::SHMUNotification};

pub struct Alert {
    pub area_desc: String,
    pub event: String,
    pub headline: String,
    pub description: String,
}

impl Alert {
    pub fn empty() -> Self {
        Self {
            area_desc: String::new(),
            event: String::new(),
            headline: String::new(),
            description: String::new(),
        }
    }

    pub fn process(&self, cfg: &Config) {
        let mut headline = String::new();
        headline.push_str(&self.area_desc);
        headline.push_str(": ");
        headline.push_str(&self.event);

        let mut body = String::new();
        body.push_str(&self.headline);
        if cfg.include_description {
            body.push_str("\n\n");
            body.push_str(&self.description);
        }

        SHMUNotification::new(&headline, &body).send();
    }
}