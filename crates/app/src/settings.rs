#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "default_sleep_seconds")]
    pub sleep_seconds: u64,
}

fn default_sleep_seconds() -> u64 {
    5
}
