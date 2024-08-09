use std::{process::Command, sync::Arc};

use async_trait::async_trait;
use sysinfo::System;
use tokio::{sync::Mutex, task::AbortHandle};

use super::{ShellProviderConfig, ShellVariables};
use crate::providers::{
  interval_provider::IntervalProvider, variables::ProviderVariables,
};

pub struct ShellProvider {
  pub config: Arc<ShellProviderConfig>,
  abort_handle: Option<AbortHandle>,
  // cmd: Arc<Command>,
}

impl ShellProvider {
  pub fn new(config: ShellProviderConfig) -> ShellProvider {
    // let mut cmd = Command::new(&config.command);
    // cmd.args(&config.args);

    ShellProvider {
      config: Arc::new(config),
      abort_handle: None,
      // cmd: cmd.into(),
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
    // self.cmd.clone()
    unimplemented!()
  }

  fn abort_handle(&self) -> &Option<AbortHandle> {
    &self.abort_handle
  }

  fn set_abort_handle(&mut self, abort_handle: AbortHandle) {
    self.abort_handle = Some(abort_handle)
  }

  async fn get_refreshed_variables(
    config: &ShellProviderConfig,
    __: &Mutex<System>,
  ) -> anyhow::Result<ProviderVariables> {
    let mut command = Command::new(&config.command);
    command.args(&config.args);

    println!("Executing command: {:?}", &config.command);
    println!("Executing args: {:?}", &config.args);

    let output = command.output().expect("failed to execute command.");

    Ok(ProviderVariables::Shell(ShellVariables {
      response: Some(String::from_utf8_lossy(&output.stdout).to_string()),
    }))
  }
}
