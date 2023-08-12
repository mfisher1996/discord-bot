use config::{Config, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Logging {
    pub name: String,
    pub level: String,
}

#[derive(Deserialize)]
pub struct Bot {
    pub token: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub logging: Logging,
    pub bot: Bot,
}

impl Settings {
    pub fn new() -> Self {
        Config::builder()
            .add_source(File::with_name("amos/settings/appsettings"))
            .build()
            .unwrap()
            .try_deserialize::<Settings>()
            .unwrap()
    }
}
