use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShellVariables {
  // TODO: See what we have to implement here
  // pub hostname: Option<String>,
  // pub os_name: Option<String>,
  // pub os_version: Option<String>,
  // pub friendly_os_version: Option<String>,
  // pub boot_time: u64,
  // pub uptime: u64,
}
