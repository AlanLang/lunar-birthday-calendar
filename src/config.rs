use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirthdayConfig {
  pub name: String,
  pub birthdays: Vec<Birthday>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Birthday {
  pub name: String,
  pub month: u8,
  pub day: u8,
}

pub fn get_config(config_str: String) -> anyhow::Result<BirthdayConfig> {
  let config = serde_json::from_str(&config_str)
    .map_err(|err| anyhow::anyhow!("Failed to parse config: {}", err));
  config
}
