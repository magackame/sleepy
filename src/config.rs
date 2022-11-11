use serde::{Deserialize, Serialize};

pub const CONFIG_FILENAME: &str = "config.json";

#[derive(Deserialize, Serialize)]
pub struct BotConfig {
    pub token: String,
}