use serde::Deserialize;

use crate::impl_interval_config;

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename = "shell")]
pub struct ShellProviderConfig {
  pub refresh_interval: u64,
  pub command: String,
  pub args: Vec<String>,
}

impl_interval_config!(ShellProviderConfig);
