use std::sync::Arc;

use async_trait::async_trait;
use sysinfo::System;
use tokio::{sync::Mutex, task::AbortHandle};

use super::{ShellProviderConfig, HostVariables};
use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

pub struct ShellProvider {
  pub config: Arc<ShellProviderConfig>,
  abort_handle: Option<AbortHandle>,
  sysinfo: Arc<Mutex<System>>,
}

impl ShellProvider {
  pub fn new(
    config: ShellProviderConfig,
    sysinfo: Arc<Mutex<System>>,
  ) -> ShellProvider {
    ShellProvider {
      config: Arc::new(config),
      abort_handle: None,
      sysinfo,
    }
  }
}

#[async_trait]
impl IntervalProvider for ShellProvider {
  type Config = ShellProviderConfig;
  type State = Mutex<System>;

  fn config(&self) -> Arc<ShellProviderConfig> {
    self.config.clone()
  }

  fn state(&self) -> Arc<Mutex<System>> {
    self.sysinfo.clone()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    _: &ShellProviderConfig,
    __: &Mutex<System>,
  ) -> anyhow::Result<ProviderVariables> {
    Ok(ProviderVariables::Host(HostVariables {
      hostname: System::host_name(),
      os_name: System::name(),
      os_version: System::os_version(),
      friendly_os_version: System::long_os_version(),
      boot_time: System::boot_time() * 1000,
      uptime: System::uptime() * 1000,
    }))
  }
}
