#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    #[serde(default)]
    pub sigterm_timeout_seconds: Option<u64>,
}
