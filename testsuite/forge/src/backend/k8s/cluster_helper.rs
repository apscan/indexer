// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::{
    get_fullnodes, get_validators, k8s_wait_genesis_strategy, k8s_wait_nodes_strategy,
    nodes_healthcheck, K8sNode, Result, DEFAULT_ROOT_KEY, FULLNODE_HAPROXY_SERVICE_SUFFIX,
    FULLNODE_SERVICE_SUFFIX, VALIDATOR_HAPROXY_SERVICE_SUFFIX, VALIDATOR_SERVICE_SUFFIX,
};
use again::RetryPolicy;
use anyhow::{bail, format_err};
use aptos_logger::info;
use aptos_sdk::types::PeerId;
use async_trait::async_trait;
use k8s_openapi::api::{
    apps::v1::{Deployment, StatefulSet},
    batch::{v1::Job, v1beta1::CronJob},
    core::v1::{ConfigMap, Namespace, PersistentVolumeClaim, Pod},
};
use kube::{
    api::{Api, DeleteParams, ListParams, Meta, ObjectMeta, Patch, PatchParams, PostParams},
    client::Client as K8sClient,
    Config, Error as KubeError,
};
use rand::Rng;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    convert::TryFrom,
    fs::File,
    io::Write,
    net::TcpListener,
    path::Path,
    process::{Command, Stdio},
    str,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tempfile::TempDir;
use thiserror::Error;
use tokio::time::Duration;

// binaries expected to be present on test runner
const HELM_BIN: &str = "helm";
pub const KUBECTL_BIN: &str = "kubectl";

// helm release names and helm chart paths
const APTOS_NODE_HELM_RELEASE_NAME: &str = "aptos-node";
const GENESIS_HELM_RELEASE_NAME: &str = "genesis";
const APTOS_NODE_HELM_CHART_PATH: &str = "terraform/helm/aptos-node";
const GENESIS_HELM_CHART_PATH: &str = "terraform/helm/genesis";

// cleanup namespaces after 30 minutes unless "keep = true"
const NAMESPACE_CLEANUP_THRESHOLD_SECS: u64 = 1800;
// Leave a buffer of around 20 minutes for test provisioning and cleanup to be done before cleaning
// up underlying resources.
pub const NAMESPACE_CLEANUP_DURATION_BUFFER_SECS: u64 = 1200;
const POD_CLEANUP_THRESHOLD_SECS: u64 = 86400;
pub const MANAGEMENT_CONFIGMAP_PREFIX: &str = "forge-management";

// We use the macros below to get around the current limitations of the
// "include_str!" macro (which loads the file content at compile time, rather
// than at runtime).

// Helm value file names.
macro_rules! APTOS_NODE_FORGE_HELM_VALUES {
    () => {
        "helm-values/aptos-node-values.yaml"
    };
}
macro_rules! GENESIS_FORGE_HELM_VALUES {
    () => {
        "helm-values/genesis-values.yaml"
    };
}
/// Gets a free port
pub fn get_free_port() -> u32 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port() as u32
}

/// Waits for the testnet's genesis job to complete, while tailing the job's logs
async fn wait_genesis_job(kube_client: &K8sClient, era: &str, kube_namespace: &str) -> Result<()> {
    aptos_retrier::retry_async(k8s_wait_genesis_strategy(), || {
        let jobs: Api<Job> = Api::namespaced(kube_client.clone(), kube_namespace);
        Box::pin(async move {
            let job_name = format!("{}-aptos-genesis-e{}", GENESIS_HELM_RELEASE_NAME, era);

            let genesis_job = jobs.get_status(&job_name).await.unwrap();

            let status = genesis_job.status.unwrap();
            info!("Genesis status: {:?}", status);
            match status.active {
                Some(_) => {
                    // try tailing the logs of the genesis job
                    // by the time this is done, we can re-evalulate its status
                    Command::new(KUBECTL_BIN)
                        .args([
                            "-n",
                            kube_namespace,
                            "logs",
                            "-f",
                            format!("job/{}", &job_name).as_str(),
                        ])
                        .status()
                        .expect("Failed to tail genesis logs");
                }
                None => info!("Genesis completed running"),
            }
            info!("Genesis status: {:?}", status);
            match status.succeeded {
                Some(_) => {
                    info!("Genesis done");
                    Ok(())
                }
                None => bail!("Genesis did not succeed"),
            }
        })
    })
    .await
}

/// Waits for a given number of HAProxy K8s Deployments to be ready
async fn wait_node_haproxy(
    kube_client: &K8sClient,
    kube_namespace: &str,
    num_haproxy: usize,
) -> Result<()> {
    aptos_retrier::retry_async(k8s_wait_nodes_strategy(), || {
        let deployments_api: Api<Deployment> = Api::namespaced(kube_client.clone(), kube_namespace);
        Box::pin(async move {
            for i in 0..num_haproxy {
                let haproxy_deployment_name =
                    format!("{}-{}-haproxy", APTOS_NODE_HELM_RELEASE_NAME, i);
                match deployments_api.get_status(&haproxy_deployment_name).await {
                    Ok(s) => {
                        let deployment_name = s.name();
                        if let Some(deployment_status) = s.status {
                            let ready_replicas = deployment_status.ready_replicas.unwrap_or(0);
                            info!(
                                "Deployment {} has {} ready_replicas",
                                deployment_name, ready_replicas
                            );
                            if ready_replicas > 0 {
                                info!("Deployment {} ready", deployment_name);
                                continue;
                            }
                        }
                        info!("Deployment {} has no status", deployment_name);
                        bail!("Deployment not ready");
                    }
                    Err(e) => {
                        info!("Failed to get deployment: {}", e);
                        bail!("Failed to get deployment: {}", e);
                    }
                }
            }
            Ok(())
        })
    })
    .await
}

/// Waits for a single K8s StatefulSet to be ready
async fn wait_stateful_set(
    kube_client: &K8sClient,
    kube_namespace: &str,
    sts_name: &str,
    desired_replicas: u64,
) -> Result<()> {
    aptos_retrier::retry_async(k8s_wait_nodes_strategy(), || {
        let sts_api: Api<StatefulSet> = Api::namespaced(kube_client.clone(), kube_namespace);
        Box::pin(async move {
            match sts_api.get_status(sts_name).await {
                Ok(s) => {
                    let sts_name = &s.name();
                    if let Some(sts_status) = s.status {
                        let ready_replicas = sts_status.ready_replicas.unwrap_or(0) as u64;
                        let replicas = sts_status.replicas as u64;
                        if ready_replicas == replicas && replicas == desired_replicas {
                            info!(
                                "StatefulSet {} has scaled to {}",
                                sts_name, desired_replicas
                            );
                            return Ok(());
                        }
                    }
                    info!(
                        "StatefulSet {} has no status. It could be Pending, ContainerCreating, CrashLoopBackoff",
                        sts_name
                    );
                    bail!("STS not ready");
                }
                Err(e) => {
                    info!("Failed to get sts: {}", e);
                    bail!("Failed to get sts: {}", e);
                }
            }
        })
    })
    .await
}

/// Waits for all given K8sNodes to be ready
async fn wait_nodes_stateful_set(
    kube_client: &K8sClient,
    kube_namespace: &str,
    nodes: &HashMap<PeerId, K8sNode>,
) -> Result<()> {
    // wait for all validators healthy
    for node in nodes.values() {
        wait_stateful_set(kube_client, kube_namespace, node.stateful_set_name(), 1).await?
    }
    Ok(())
}

// TODO: set validator image tag by kube api rather than helm
pub fn set_validator_image_tag(
    _validator_name: String,
    _image_tag: String,
    _kube_namespace: String,
) -> Result<()> {
    todo!()
}

/// Deletes a collection of resources in k8s as part of aptos-node
async fn delete_k8s_collection<T: Clone + DeserializeOwned + Meta>(
    api: Api<T>,
    name: &'static str,
    label_selector: &str,
) -> Result<()> {
    match api
        .delete_collection(
            &DeleteParams::default(),
            &ListParams::default().labels(label_selector),
        )
        .await?
    {
        either::Left(list) => {
            let names: Vec<_> = list.iter().map(Meta::name).collect();
            info!("Deleting collection of {}: {:?}", name, names);
        }
        either::Right(status) => {
            info!("Deleted collection of {}: status={:?}", name, status);
        }
    }

    Ok(())
}

/// Delete existing k8s resources in the namespace. This is essentially helm uninstall but lighter weight
pub(crate) async fn delete_k8s_resources(client: K8sClient, kube_namespace: &str) -> Result<()> {
    // selector for the helm chart
    let aptos_node_helm_selector = "app.kubernetes.io/part-of=aptos-node";
    let testnet_addons_helm_selector = "app.kubernetes.io/part-of=testnet-addons";
    let genesis_helm_selector = "app.kubernetes.io/part-of=aptos-genesis";

    // delete all deployments and statefulsets
    // cross this with all the compute resources created by aptos-node helm chart
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), kube_namespace);
    let stateful_sets: Api<StatefulSet> = Api::namespaced(client.clone(), kube_namespace);
    let pvcs: Api<PersistentVolumeClaim> = Api::namespaced(client.clone(), kube_namespace);
    let jobs: Api<Job> = Api::namespaced(client.clone(), kube_namespace);
    let cronjobs: Api<CronJob> = Api::namespaced(client.clone(), kube_namespace);
    // service deletion by label selector is not supported in this version of k8s api
    // let services: Api<Service> = Api::namespaced(client.clone(), kube_namespace);

    for selector in &[
        aptos_node_helm_selector,
        testnet_addons_helm_selector,
        genesis_helm_selector,
    ] {
        info!("Deleting k8s resources with selector: {}", selector);
        delete_k8s_collection(deployments.clone(), "Deployments", selector).await?;
        delete_k8s_collection(stateful_sets.clone(), "StatefulSets", selector).await?;
        delete_k8s_collection(pvcs.clone(), "PersistentVolumeClaims", selector).await?;
        delete_k8s_collection(jobs.clone(), "Jobs", selector).await?;
        delete_k8s_collection(cronjobs.clone(), "CronJobs", selector).await?;
        // delete_k8s_collection(services.clone(), "Services", selector).await?;
    }

    Ok(())
}

/// Deletes all Forge resources from the given namespace. If the namespace is "default", delete the management configmap
/// as well as all compute resources. If the namespace is a Forge namespace (has the "forge-*" prefix), then simply delete
/// the entire namespace
async fn delete_k8s_cluster(kube_namespace: String) -> Result<()> {
    let client: K8sClient = create_k8s_client().await;

    // if operating on the default namespace,
    match kube_namespace.as_str() {
        "default" => {
            // delete the management configmap
            let configmap: Api<ConfigMap> = Api::namespaced(client.clone(), &kube_namespace);
            let management_configmap_name =
                format!("{}-{}", MANAGEMENT_CONFIGMAP_PREFIX, &kube_namespace);
            match configmap
                .delete(&management_configmap_name, &DeleteParams::default())
                .await
            {
                Ok(_) => info!(
                    "Deleted default management configmap: {}",
                    &management_configmap_name
                ),
                // if configmap not found, assume it's already been deleted and make clean-up idempotent
                Err(KubeError::Api(api_err)) => {
                    if api_err.code == 404 {
                        info!(
                            "Could not find configmap {}, continuing",
                            &management_configmap_name
                        );
                    } else {
                        bail!(api_err);
                    }
                }
                Err(e) => bail!(e),
            };
            delete_k8s_resources(client, "default").await?;
        }
        s if s.starts_with("forge") => {
            let namespaces: Api<Namespace> = Api::all(client);
            namespaces
                .delete(&kube_namespace, &DeleteParams::default())
                .await?
                .map_left(|namespace| info!("Deleting namespace {}: {:?}", s, namespace.status))
                .map_right(|status| info!("Deleted namespace {}: {:?}", s, status));
        }
        _ => {
            bail!(
                "Invalid kubernetes namespace provided: {}. Use forge-*",
                kube_namespace
            );
        }
    }

    Ok(())
}

fn upgrade_helm_release(
    release_name: String,
    helm_chart: String,
    options: &[String],
    kube_namespace: String,
) -> Result<()> {
    // Check to make sure helm_chart exists
    let helm_chart_path = Path::new(&helm_chart);
    if !helm_chart_path.exists() {
        bail!(
            "Helm chart {} does not exist, try running from the repo root",
            helm_chart
        );
    }

    // only create cluster-level resources once
    let psp_values = match kube_namespace.as_str() {
        "default" => "podSecurityPolicy=true",
        _ => "podSecurityPolicy=false",
    };
    let upgrade_base_args = [
        "upgrade".to_string(),
        // "--debug".to_string(),
        "--install".to_string(),
        // // force replace if necessary
        // "--force".to_string(),
        // in a new namespace
        "--create-namespace".to_string(),
        "--namespace".to_string(),
        kube_namespace,
        // upgrade
        release_name.clone(),
        helm_chart.clone(),
        // reuse old values
        "--reuse-values".to_string(),
        "--history-max".to_string(),
        "2".to_string(),
    ];
    let upgrade_override_args = ["--set".to_string(), psp_values.to_string()];
    let upgrade_args = [&upgrade_base_args, options, &upgrade_override_args].concat();
    info!("{:?}", upgrade_args);
    let upgrade_output = Command::new(HELM_BIN)
        .stdout(Stdio::inherit())
        .args(&upgrade_args)
        .output()
        .unwrap_or_else(|_| {
            panic!(
                "failed to helm upgrade release {} with chart {}",
                release_name, helm_chart
            )
        });
    if !upgrade_output.status.success() {
        bail!(format!(
            "Upgrade not completed: {}",
            String::from_utf8(upgrade_output.stderr).unwrap()
        ));
    }

    Ok(())
}

// TODO: upgrade via kube api
#[allow(dead_code)]
fn upgrade_validator(
    _validator_name: String,
    _options: &[String],
    _kube_namespace: String,
) -> Result<()> {
    todo!()
}

fn upgrade_aptos_node_helm(options: &[String], kube_namespace: String) -> Result<()> {
    upgrade_helm_release(
        APTOS_NODE_HELM_RELEASE_NAME.to_string(),
        APTOS_NODE_HELM_CHART_PATH.to_string(),
        options,
        kube_namespace,
    )
}

// runs helm upgrade on the installed aptos-genesis release named "genesis"
// if a new "era" is specified, a new genesis will be created, and old resources will be destroyed
fn upgrade_genesis_helm(options: &[String], kube_namespace: String) -> Result<()> {
    upgrade_helm_release(
        GENESIS_HELM_RELEASE_NAME.to_string(),
        GENESIS_HELM_CHART_PATH.to_string(),
        options,
        kube_namespace,
    )
}

pub async fn uninstall_testnet_resources(kube_namespace: String) -> Result<()> {
    // delete kubernetes resources
    delete_k8s_cluster(kube_namespace.clone()).await?;
    info!(
        "aptos-node resources for Forge removed in namespace: {}",
        kube_namespace
    );

    Ok(())
}

fn generate_new_era() -> String {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    format!("forge{}", r)
}

pub async fn install_testnet_resources(
    kube_namespace: String,
    num_validators: usize,
    num_fullnodes: usize,
    node_image_tag: String,
    genesis_image_tag: String,
    genesis_modules_path: Option<String>,
    use_port_forward: bool,
    enable_haproxy: bool,
) -> Result<(HashMap<PeerId, K8sNode>, HashMap<PeerId, K8sNode>)> {
    let kube_client = create_k8s_client().await;

    // get deployment-specific helm values and cache it
    let tmp_dir = TempDir::new().expect("Could not create temp dir");
    let aptos_node_values_file = dump_helm_values_to_file(APTOS_NODE_HELM_RELEASE_NAME, &tmp_dir)?;
    let genesis_values_file = dump_helm_values_to_file(GENESIS_HELM_RELEASE_NAME, &tmp_dir)?;

    // generate a random era to wipe the network state
    let new_era = generate_new_era();

    // get forge override helm values and cache it
    let aptos_node_forge_helm_values_yaml = format!(
        include_str!(APTOS_NODE_FORGE_HELM_VALUES!()),
        num_validators = num_validators,
        num_fullnodes = num_fullnodes,
        era = &new_era,
        image_tag = &node_image_tag,
        enable_haproxy = enable_haproxy,
        namespace = &kube_namespace,
    );
    let aptos_node_forge_values_file = dump_string_to_file(
        "aptos-node-values.yaml".to_string(),
        aptos_node_forge_helm_values_yaml,
        &tmp_dir,
    )?;
    let validator_internal_host_suffix = if enable_haproxy {
        VALIDATOR_HAPROXY_SERVICE_SUFFIX
    } else {
        VALIDATOR_SERVICE_SUFFIX
    };
    let fullnode_internal_host_suffix = if enable_haproxy {
        FULLNODE_HAPROXY_SERVICE_SUFFIX
    } else {
        FULLNODE_SERVICE_SUFFIX
    };

    let genesis_forge_helm_values_yaml = format!(
        include_str!(GENESIS_FORGE_HELM_VALUES!()),
        num_validators = num_validators,
        image_tag = &genesis_image_tag,
        era = &new_era,
        root_key = DEFAULT_ROOT_KEY,
        validator_internal_host_suffix = validator_internal_host_suffix,
        fullnode_internal_host_suffix = fullnode_internal_host_suffix,
        namespace = &kube_namespace,
    );
    let genesis_forge_values_file = dump_string_to_file(
        "genesis-values.yaml".to_string(),
        genesis_forge_helm_values_yaml,
        &tmp_dir,
    )?;

    // combine all helm values
    let aptos_node_upgrade_options = vec![
        // use the old values
        "-f".to_string(),
        aptos_node_values_file,
        "-f".to_string(),
        aptos_node_forge_values_file,
    ];

    let mut genesis_upgrade_options = vec![
        // use the old values
        "-f".to_string(),
        genesis_values_file,
        "-f".to_string(),
        genesis_forge_values_file,
    ];

    // run genesis from the directory in aptos/init image
    if let Some(genesis_modules_path) = genesis_modules_path {
        genesis_upgrade_options.extend([
            "--set".to_string(),
            format!("genesis.moveModulesDir={}", genesis_modules_path),
        ]);
    }

    // upgrade genesis
    upgrade_genesis_helm(genesis_upgrade_options.as_slice(), kube_namespace.clone())?;

    // wait for genesis to run again, and get the updated validators
    wait_genesis_job(&kube_client, &new_era, &kube_namespace).await?;

    // TODO(rustielin): get the helm releases to be consistent
    upgrade_aptos_node_helm(
        aptos_node_upgrade_options.as_slice(),
        kube_namespace.clone(),
    )?;

    let (validators, fullnodes) = collect_running_nodes(
        &kube_client,
        kube_namespace,
        use_port_forward,
        enable_haproxy,
    )
    .await?;

    Ok((validators, fullnodes))
}

/// Collect the running nodes in the network into K8sNodes
pub async fn collect_running_nodes(
    kube_client: &K8sClient,
    kube_namespace: String,
    use_port_forward: bool,
    enable_haproxy: bool,
) -> Result<(HashMap<PeerId, K8sNode>, HashMap<PeerId, K8sNode>)> {
    // get all validators
    let validators = get_validators(
        kube_client.clone(),
        &kube_namespace,
        use_port_forward,
        enable_haproxy,
    )
    .await
    .unwrap();

    // wait for all validator STS to spin up
    wait_nodes_stateful_set(kube_client, &kube_namespace, &validators).await?;

    if enable_haproxy {
        wait_node_haproxy(kube_client, &kube_namespace, validators.len()).await?;
    }

    // get all fullnodes
    let fullnodes = get_fullnodes(
        kube_client.clone(),
        &kube_namespace,
        use_port_forward,
        enable_haproxy,
    )
    .await
    .unwrap();

    wait_nodes_stateful_set(kube_client, &kube_namespace, &fullnodes).await?;

    let nodes = validators
        .values()
        .chain(fullnodes.values())
        .collect::<Vec<&K8sNode>>();

    // start port-forward for each of the nodes
    if use_port_forward {
        for node in nodes.iter() {
            node.port_forward_rest_api()?;
            // assume this will always succeed???
        }
    }

    nodes_healthcheck(nodes).await?;
    Ok((validators, fullnodes))
}

pub async fn create_k8s_client() -> K8sClient {
    // get the client from the local kube context
    // TODO(rustielin|geekflyer): use proxy or port-forward to make REST API available
    let config_infer = Config::infer().await.unwrap();
    K8sClient::try_from(config_infer).unwrap()
}

// TODO: replace this with rust kube api call
pub async fn scale_stateful_set_replicas(
    sts_name: &str,
    kube_namespace: &str,
    replica_num: u64,
) -> Result<()> {
    let kube_client = create_k8s_client().await;
    let stateful_set_api: Api<StatefulSet> = Api::namespaced(kube_client.clone(), kube_namespace);
    let pp = PatchParams::apply("forge").force();
    let patch = serde_json::json!({
        "apiVersion": "apps/v1",
        "kind": "StatefulSet",
        "metadata": {
            "name": sts_name,
        },
        "spec": {
            "replicas": replica_num,
        }
    });
    let patch = Patch::Apply(&patch);
    stateful_set_api.patch(sts_name, &pp, &patch).await?;
    wait_stateful_set(&kube_client, kube_namespace, sts_name, replica_num).await?;

    Ok(())
}

/// Gets the result of helm status command as JSON
fn get_helm_status(helm_release_name: &str) -> Result<Value> {
    let status_args = [
        "status",
        helm_release_name,
        "--namespace",
        "default",
        "-o",
        "json",
    ];
    info!("{:?}", status_args);
    let raw_helm_values = Command::new(HELM_BIN)
        .args(&status_args)
        .output()
        .unwrap_or_else(|_| panic!("Failed to helm status {}", helm_release_name));

    let helm_values = String::from_utf8(raw_helm_values.stdout).unwrap();
    serde_json::from_str(&helm_values).map_err(|e| {
        format_err!(
            "Failed to deserialize helm values. Check if release {} exists: {}",
            helm_release_name,
            e
        )
    })
}

/// Dumps the given String contents into a file at the given temp directory
pub fn dump_string_to_file(
    file_name: String,
    content: String,
    tmp_dir: &TempDir,
) -> Result<String> {
    let file_path = tmp_dir.path().join(file_name.clone());
    info!("Wrote content to: {:?}", &file_path);
    let mut file = File::create(file_path).expect("Could not create file in temp dir");
    file.write_all(&content.into_bytes())
        .expect("Could not write to file");
    let file_path_str = tmp_dir.path().join(file_name).display().to_string();
    Ok(file_path_str)
}

fn dump_helm_values_to_file(helm_release_name: &str, tmp_dir: &TempDir) -> Result<String> {
    // get aptos-node values
    let v: Value = get_helm_status(helm_release_name).unwrap();
    let config = &v["config"];
    let content = config.to_string();
    let file_name = format!("{}_status.json", helm_release_name);

    dump_string_to_file(file_name, content, tmp_dir)
}

struct K8sNamespacesApi {
    api: Api<Namespace>,
}

#[async_trait]
trait CreateNamespace: Send + Sync {
    async fn create(&self, pp: &PostParams, namespace: &Namespace) -> Result<Namespace, KubeError>;
}

#[async_trait]
impl CreateNamespace for Api<Namespace> {
    async fn create(&self, pp: &PostParams, namespace: &Namespace) -> Result<Namespace, KubeError> {
        self.create(pp, namespace).await
    }
}

impl K8sNamespacesApi {
    fn from_client(kube_client: K8sClient) -> Self {
        K8sNamespacesApi {
            api: Api::all(kube_client),
        }
    }
}

#[async_trait]
impl CreateNamespace for K8sNamespacesApi {
    async fn create(&self, pp: &PostParams, namespace: &Namespace) -> Result<Namespace, KubeError> {
        self.api.create(pp, namespace).await
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
enum ApiError {
    RetryableError(String),
    FinalError(String),
}

async fn create_namespace(
    namespace_creator: Arc<dyn CreateNamespace>,
    kube_namespace: String,
) -> Result<(), ApiError> {
    let kube_namespace_name = kube_namespace.clone();
    let namespace = Namespace {
        metadata: ObjectMeta {
            name: Some(kube_namespace_name.clone()),
            ..ObjectMeta::default()
        },
        spec: None,
        status: None,
    };
    if let Err(KubeError::Api(api_err)) = namespace_creator
        .create(&PostParams::default(), &namespace)
        .await
    {
        if api_err.code == 409 {
            info!(
                "Namespace {} already exists, continuing with it",
                &kube_namespace_name
            );
        } else if api_err.code == 401 {
            return Err(ApiError::FinalError(
                "Unauthorized, did you authorize with kubernetes? \
                    Try running `kubectl get current-context`"
                    .to_string(),
            ));
        } else {
            return Err(ApiError::RetryableError(format!(
                "Failed to use existing namespace {}: {:?}",
                &kube_namespace_name, api_err
            )));
        }
    }
    Ok(())
}

pub async fn create_management_configmap(
    kube_namespace: String,
    keep: bool,
    cleanup_duration: Duration,
) -> Result<()> {
    let kube_client = create_k8s_client().await;
    let namespaces_api = Arc::new(K8sNamespacesApi::from_client(kube_client.clone()));
    let other_kube_namespace = kube_namespace.clone();

    // try to create a new namespace
    // * if it errors with 409, the namespace exists already and we should use it
    // * if it errors with 403, the namespace is likely in the process of being terminated, so try again
    RetryPolicy::exponential(Duration::from_millis(1000))
        .with_max_delay(Duration::from_millis(10 * 60 * 1000))
        .retry_if(
            move || create_namespace(namespaces_api.clone(), other_kube_namespace.clone()),
            |e: &ApiError| matches!(e, ApiError::RetryableError(_)),
        )
        .await?;

    let configmap: Api<ConfigMap> = Api::namespaced(kube_client.clone(), &kube_namespace);

    let management_configmap_name = format!("{}-{}", MANAGEMENT_CONFIGMAP_PREFIX, &kube_namespace);
    let mut data: BTreeMap<String, String> = BTreeMap::new();
    let start = SystemTime::now();
    let cleanup_time = (start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        + cleanup_duration)
        .as_secs();
    data.insert("keep".to_string(), keep.to_string());
    data.insert("cleanup".to_string(), cleanup_time.to_string());

    let config = ConfigMap {
        binary_data: None,
        data: Some(data.clone()),
        metadata: ObjectMeta {
            name: Some(management_configmap_name.clone()),
            ..ObjectMeta::default()
        },
    };
    if let Err(KubeError::Api(api_err)) = configmap.create(&PostParams::default(), &config).await {
        if api_err.code == 409 {
            info!(
                "Configmap {} already exists, continuing with it",
                &management_configmap_name
            );
        } else {
            bail!(
                "Failed to use existing management configmap {}: {:?}",
                &kube_namespace,
                api_err
            );
        }
    } else {
        info!(
            "Created configmap {} with data {:?}",
            management_configmap_name, data
        );
    }

    Ok(())
}

pub async fn cleanup_cluster_with_management() -> Result<()> {
    let kube_client = create_k8s_client().await;
    let start = SystemTime::now();
    let time_since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let pods_api: Api<Pod> = Api::namespaced(kube_client.clone(), "default");
    let lp = ListParams::default().labels("run");

    // delete all forge test pods over a threshold age
    let pods = pods_api
        .list(&lp)
        .await?
        .items
        .into_iter()
        .filter(|pod| {
            let pod_name = pod.name();
            info!("Got pod {}", pod_name);
            if let Some(time) = &pod.metadata.creation_timestamp {
                let pod_creation_time = time.0.timestamp() as u64;
                let pod_uptime = time_since_the_epoch - pod_creation_time;
                info!(
                    "Pod {} has lived for {}/{} seconds",
                    pod_name, pod_uptime, POD_CLEANUP_THRESHOLD_SECS
                );
                if pod_uptime > POD_CLEANUP_THRESHOLD_SECS {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<Pod>>();
    for pod in pods {
        let pod_name = pod.name();
        info!("Deleting pod {}", pod_name);
        pods_api.delete(&pod_name, &DeleteParams::default()).await?;
    }

    // delete all forge testnets over a threshold age using their management configmaps
    // unless they are explicitly set with "keep = true"
    let configmaps_api: Api<ConfigMap> = Api::all(kube_client.clone());
    let lp = ListParams::default();
    let configmaps = configmaps_api
        .list(&lp)
        .await?
        .items
        .into_iter()
        .filter(|configmap| {
            let configmap_name = configmap.name();
            let configmap_namespace = configmap.namespace().unwrap();
            if !configmap_name.contains(MANAGEMENT_CONFIGMAP_PREFIX) {
                return false;
            }
            if let Some(data) = &configmap.data {
                info!("Got configmap {} with data: {:?}", &configmap_name, data);
                return check_namespace_for_cleanup(
                    data,
                    configmap_namespace,
                    time_since_the_epoch,
                );
            }
            false
        })
        .collect::<Vec<ConfigMap>>();
    for configmap in configmaps {
        let namespace = configmap.namespace().unwrap();
        uninstall_testnet_resources(namespace).await?;
    }

    Ok(())
}

fn check_namespace_for_cleanup(
    data: &BTreeMap<String, String>,
    namespace: String,
    time_since_the_epoch: u64,
) -> bool {
    let keep: bool = data.get("keep").unwrap().parse().unwrap();
    if keep {
        info!("Explicitly keeping namespace {}", namespace);
        return false;
    }
    if data.get("cleanup").is_none() {
        // This is needed for backward compatibility where older namespaces created
        // don't have "cleanup" time set. Delete this code once we roll out the cleanup
        // feature fully
        let start: u64 = data.get("start").unwrap().parse().unwrap();
        let namespace_uptime = time_since_the_epoch - start;
        info!(
            "Namespace {} has lived for {}/{} seconds",
            namespace, namespace_uptime, NAMESPACE_CLEANUP_THRESHOLD_SECS
        );
        if keep {
            info!("Explicitly keeping namespace {}", namespace);
            return false;
        }
        if namespace_uptime > NAMESPACE_CLEANUP_THRESHOLD_SECS {
            return true;
        }
    } else {
        // TODO(rustielin): come up with some sane values for namespaces
        let cleanup_time_since_epoch: u64 = data.get("cleanup").unwrap().parse().unwrap();
        info!(
            "Namespace {} has remaining {} seconds before cleanup",
            namespace,
            cleanup_time_since_epoch - time_since_the_epoch
        );

        if cleanup_time_since_epoch <= time_since_the_epoch {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::http::StatusCode;
    use kube::error::ErrorResponse;

    struct FailedNamespacesApi {
        status_code: u16,
    }

    impl FailedNamespacesApi {
        fn from_status_code(status_code: u16) -> Self {
            FailedNamespacesApi { status_code }
        }
    }

    #[async_trait]
    impl CreateNamespace for FailedNamespacesApi {
        async fn create(
            &self,
            _pp: &PostParams,
            _namespace: &Namespace,
        ) -> Result<Namespace, KubeError> {
            let status = StatusCode::from_u16(self.status_code).unwrap();
            Err(KubeError::Api(ErrorResponse {
                status: status.to_string(),
                code: status.as_u16(),
                message: "Failed to create namespace".to_string(),
                reason: "Failed to parse error data".into(),
            }))
        }
    }

    #[tokio::test]
    async fn test_create_namespace_final_error() {
        let namespace_creator = Arc::new(FailedNamespacesApi::from_status_code(401));
        let result = create_namespace(namespace_creator, "banana".to_string()).await;
        match result {
            Err(ApiError::FinalError(_)) => {}
            _ => panic!("Expected final error"),
        }
    }

    #[tokio::test]
    async fn test_create_namespace_retryable_error() {
        let namespace_creator = Arc::new(FailedNamespacesApi::from_status_code(403));
        let result = create_namespace(namespace_creator, "banana".to_string()).await;
        match result {
            Err(ApiError::RetryableError(_)) => {}
            _ => panic!("Expected retryable error"),
        }
    }

    #[tokio::test]
    async fn test_check_namespace_for_cleanup() {
        let start = SystemTime::now();
        let time_since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut data = BTreeMap::new();

        // Ensure very old run without keep is cleaned up.
        data.insert("keep".to_string(), "false".to_string());
        data.insert("start".to_string(), "0".to_string());

        assert!(check_namespace_for_cleanup(
            &data,
            "foo".to_string(),
            time_since_the_epoch
        ));

        // Ensure old run with keep is not cleaned up.
        data.insert("keep".to_string(), "true".to_string());
        data.insert("start".to_string(), "0".to_string());

        assert!(!check_namespace_for_cleanup(
            &data,
            "foo".to_string(),
            time_since_the_epoch
        ));

        // Ensure very old run without keep is cleaned up.
        data.insert("keep".to_string(), "false".to_string());
        data.insert("cleanup".to_string(), "20".to_string());

        assert!(check_namespace_for_cleanup(
            &data,
            "foo".to_string(),
            time_since_the_epoch
        ));

        // Ensure old run with keep is not cleaned up.
        data.insert("keep".to_string(), "true".to_string());
        data.insert("cleanup".to_string(), "20".to_string());

        assert!(!check_namespace_for_cleanup(
            &data,
            "foo".to_string(),
            time_since_the_epoch
        ));

        // Ensure a run with clean up some time in future is not cleaned up.
        data.insert("keep".to_string(), "false".to_string());
        let cleanup_time = (start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            + Duration::from_secs(3600))
        .as_secs();
        data.insert("cleanup".to_string(), cleanup_time.to_string());

        assert!(!check_namespace_for_cleanup(
            &data,
            "foo".to_string(),
            time_since_the_epoch
        ));
    }
}
