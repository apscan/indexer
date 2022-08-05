// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{FullNode, HealthCheckError, LocalVersion, Node, NodeExt, Validator, Version};
use anyhow::{anyhow, ensure, Context, Result};
use aptos_config::config::NodeConfig;
use aptos_logger::debug;
use aptos_sdk::types::{account_address::AccountAddress, PeerId};
use std::{
    env,
    fs::{self, OpenOptions},
    path::PathBuf,
    process::{Child, Command},
    str::FromStr,
};
use url::Url;

#[derive(Debug)]
struct Process(Child);

impl Drop for Process {
    // When the Process struct goes out of scope we need to kill the child process
    fn drop(&mut self) {
        // check if the process has already been terminated
        match self.0.try_wait() {
            // The child process has already terminated, perhaps due to a crash
            Ok(Some(_)) => {}

            // The process is still running so we need to attempt to kill it
            _ => {
                self.0.kill().expect("Process wasn't running");
                self.0.wait().unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct LocalNode {
    version: LocalVersion,
    process: Option<Process>,
    name: String,
    peer_id: AccountAddress,
    directory: PathBuf,
    config: NodeConfig,
}

impl LocalNode {
    pub fn new(version: LocalVersion, name: String, directory: PathBuf) -> Result<Self> {
        let config_path = directory.join("node.yaml");
        let config = NodeConfig::load(&config_path)
            .with_context(|| format!("Failed to load NodeConfig from file: {:?}", config_path))?;
        let peer_id = config
            .peer_id()
            .ok_or_else(|| anyhow!("unable to retrieve PeerId from config"))?;

        Ok(Self {
            version,
            process: None,
            name,
            peer_id,
            directory,
            config,
        })
    }

    pub fn config_path(&self) -> PathBuf {
        self.directory.join("node.yaml")
    }

    pub fn log_path(&self) -> PathBuf {
        self.directory.join("log")
    }

    pub fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start(&mut self) -> Result<()> {
        ensure!(self.process.is_none(), "node {} already running", self.name);

        // Ensure log file exists
        let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(self.log_path())?;

        // Start node process
        let mut node_command = Command::new(self.version.bin());
        node_command
            .current_dir(&self.directory)
            .arg("-f")
            .arg(self.config_path());
        if env::var("RUST_LOG").is_err() {
            // Only set our RUST_LOG if its not present in environment
            node_command.env("RUST_LOG", "debug");
        }
        node_command.stdout(log_file.try_clone()?).stderr(log_file);
        let process = node_command.spawn().with_context(|| {
            format!(
                "Error launching node process with binary: {:?}",
                self.version.bin()
            )
        })?;

        println!(
            "Started node {:?} (PID: {:?}) with command: {:?}",
            self.name,
            process.id(),
            node_command
        );
        self.process = Some(Process(process));

        Ok(())
    }

    pub fn stop(&mut self) {
        self.process = None;
    }

    pub fn port(&self) -> u16 {
        self.config.api.address.port()
    }

    pub fn inspection_service_port(&self) -> u16 {
        self.config.inspection_service.port
    }

    pub fn config(&self) -> &NodeConfig {
        &self.config
    }

    pub(crate) fn config_mut(&mut self) -> &mut NodeConfig {
        &mut self.config
    }

    pub fn upgrade(&mut self, version: LocalVersion) -> Result<()> {
        self.stop();
        self.version = version;
        self.start()
    }

    pub fn get_log_contents(&self) -> Result<String> {
        fs::read_to_string(self.log_path()).map_err(Into::into)
    }

    pub async fn health_check(&mut self) -> Result<(), HealthCheckError> {
        debug!("Health check on node '{}'", self.name);

        if let Some(p) = &mut self.process {
            match p.0.try_wait() {
                // This would mean the child process has crashed
                Ok(Some(status)) => {
                    let error = format!("Node '{}' crashed with: {}", self.name, status);
                    return Err(HealthCheckError::NotRunning(error));
                }

                // This is the case where the node is still running
                Ok(None) => {}

                // Some other unknown error
                Err(e) => {
                    return Err(HealthCheckError::Unknown(e.into()));
                }
            }
        } else {
            let error = format!("Node '{}' is stopped", self.name);
            return Err(HealthCheckError::NotRunning(error));
        }

        self.inspection_client()
            .get_node_metrics()
            .await
            .map(|_| ())
            .map_err(HealthCheckError::Failure)?;

        self.rest_client()
            .get_ledger_information()
            .await
            .map(|_| ())
            .map_err(HealthCheckError::Failure)
    }
}

#[async_trait::async_trait]
impl Node for LocalNode {
    fn peer_id(&self) -> PeerId {
        self.peer_id()
    }

    fn name(&self) -> &str {
        self.name()
    }

    fn version(&self) -> Version {
        self.version.version()
    }

    fn rest_api_endpoint(&self) -> Url {
        let ip = self.config().api.address.ip();
        let port = self.config().api.address.port();
        Url::from_str(&format!("http://{}:{}", ip, port)).expect("Invalid URL.")
    }

    fn inspection_service_endpoint(&self) -> Url {
        Url::parse(&format!(
            "http://localhost:{}",
            self.inspection_service_port()
        ))
        .unwrap()
    }

    fn config(&self) -> &NodeConfig {
        self.config()
    }

    async fn start(&mut self) -> Result<()> {
        self.start()
    }

    async fn stop(&mut self) -> Result<()> {
        self.stop();
        Ok(())
    }

    fn clear_storage(&mut self) -> Result<()> {
        todo!()
    }

    async fn health_check(&mut self) -> Result<(), HealthCheckError> {
        self.health_check().await
    }

    fn counter(&self, _counter: &str, _port: u64) -> Result<f64> {
        todo!()
    }

    // local node does not need to expose metric end point
    fn expose_metric(&self) -> Result<u64> {
        Ok(0)
    }
}

impl Validator for LocalNode {}
impl FullNode for LocalNode {}
