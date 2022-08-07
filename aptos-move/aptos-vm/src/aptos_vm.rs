// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::delta_ext::TransactionOutputExt;
use crate::{
    adapter_common,
    adapter_common::{
        discard_error_output, discard_error_vm_status, validate_signature_checked_transaction,
        validate_signed_transaction, PreprocessedTransaction, VMAdapter,
    },
    aptos_vm_impl::{get_transaction_output, AptosVMImpl, AptosVMInternals},
    counters::*,
    data_cache::{AsMoveResolver, StateViewCache},
    errors::expect_only_successful_execution,
    logging::AdapterLogSchema,
    move_vm_ext::{MoveResolverExt, SessionExt, SessionId},
    system_module_names::*,
    transaction_arg_validation,
    transaction_metadata::TransactionMetadata,
    VMExecutor, VMValidator,
};
use anyhow::Result;
use aptos_crypto::HashValue;
use aptos_gas::AptosGasMeter;
use aptos_logger::prelude::*;
use aptos_module_verifier::module_init::verify_module_init_function;
use aptos_state_view::StateView;
use aptos_types::account_config::new_block_event_key;
use aptos_types::{
    account_config,
    block_metadata::BlockMetadata,
    on_chain_config::{new_epoch_event_key, GasSchedule, Version},
    transaction::{
        ChangeSet, ExecutionStatus, ModuleBundle, SignatureCheckedTransaction, SignedTransaction,
        Transaction, TransactionOutput, TransactionPayload, TransactionStatus, VMValidatorResult,
        WriteSetPayload,
    },
    vm_status::{StatusCode, VMStatus},
    write_set::{WriteSet, WriteSetMut},
};
use fail::fail_point;
use framework::natives::code::PublishRequest;
use move_deps::{
    move_binary_format::{
        access::ModuleAccess,
        errors::{verification_error, Location, PartialVMError, VMResult},
        CompiledModule, IndexKind,
    },
    move_core_types::{
        account_address::AccountAddress,
        ident_str,
        language_storage::ModuleId,
        transaction_argument::convert_txn_args,
        value::{serialize_values, MoveValue},
    },
    move_vm_types::gas::UnmeteredGasMeter,
};
use num_cpus;
use once_cell::sync::OnceCell;
use std::collections::BTreeSet;
use std::{
    cmp::min,
    collections::HashSet,
    convert::{AsMut, AsRef},
    sync::Arc,
};

static EXECUTION_CONCURRENCY_LEVEL: OnceCell<usize> = OnceCell::new();
static NUM_PROOF_READING_THREADS: OnceCell<usize> = OnceCell::new();

#[derive(Clone)]
pub struct AptosVM(pub(crate) AptosVMImpl);

struct AptosSimulationVM(AptosVM);

impl AptosVM {
    pub fn new<S: StateView>(state: &S) -> Self {
        Self(AptosVMImpl::new(state))
    }

    pub fn new_for_validation<S: StateView>(state: &S) -> Self {
        info!(
            AdapterLogSchema::new(state.id(), 0),
            "Adapter created for Validation"
        );
        Self::new(state)
    }

    pub fn init_with_config(version: Version, gas_schedule: GasSchedule) -> Self {
        info!("Adapter restarted for Validation");
        AptosVM(AptosVMImpl::init_with_config(version, gas_schedule))
    }

    /// Sets execution concurrency level when invoked the first time.
    pub fn set_concurrency_level_once(mut concurrency_level: usize) {
        concurrency_level = min(concurrency_level, num_cpus::get());
        // Only the first call succeeds, due to OnceCell semantics.
        EXECUTION_CONCURRENCY_LEVEL.set(concurrency_level).ok();
    }

    /// Get the concurrency level if already set, otherwise return default 1
    /// (sequential execution).
    pub fn get_concurrency_level() -> usize {
        match EXECUTION_CONCURRENCY_LEVEL.get() {
            Some(concurrency_level) => *concurrency_level,
            None => 1,
        }
    }

    /// Sets the # of async proof reading threads.
    pub fn set_num_proof_reading_threads_once(mut num_threads: usize) {
        // TODO(grao): Do more analysis to tune this magic number.
        num_threads = min(num_threads, 256);
        // Only the first call succeeds, due to OnceCell semantics.
        NUM_PROOF_READING_THREADS.set(num_threads).ok();
    }

    /// Returns the # of async proof reading threads if already set, otherwise return default value
    /// (32).
    pub fn get_num_proof_reading_threads() -> usize {
        match NUM_PROOF_READING_THREADS.get() {
            Some(num_threads) => *num_threads,
            None => 32,
        }
    }

    pub fn internals(&self) -> AptosVMInternals {
        AptosVMInternals::new(&self.0)
    }

    /// Load a module into its internal MoveVM's code cache.
    pub fn load_module<S: MoveResolverExt>(
        &self,
        module_id: &ModuleId,
        state: &S,
    ) -> VMResult<Arc<CompiledModule>> {
        self.0.load_module(module_id, state)
    }

    /// Generates a transaction output for a transaction that encountered errors during the
    /// execution process. This is public for now only for tests.
    pub fn failed_transaction_cleanup<S: MoveResolverExt>(
        &self,
        error_code: VMStatus,
        gas_meter: &mut AptosGasMeter,
        txn_data: &TransactionMetadata,
        storage: &S,
        log_context: &AdapterLogSchema,
    ) -> TransactionOutputExt {
        self.failed_transaction_cleanup_and_keep_vm_status(
            error_code,
            gas_meter,
            txn_data,
            storage,
            log_context,
        )
        .1
    }

    fn failed_transaction_cleanup_and_keep_vm_status<S: MoveResolverExt>(
        &self,
        error_code: VMStatus,
        gas_meter: &mut AptosGasMeter,
        txn_data: &TransactionMetadata,
        storage: &S,
        log_context: &AdapterLogSchema,
    ) -> (VMStatus, TransactionOutputExt) {
        let mut session = self.0.new_session(storage, SessionId::txn_meta(txn_data));
        match TransactionStatus::from(error_code.clone()) {
            TransactionStatus::Keep(status) => {
                // The transaction should be charged for gas, so run the epilogue to do that.
                // This is running in a new session that drops any side effects from the
                // attempted transaction (e.g., spending funds that were needed to pay for gas),
                // so even if the previous failure occurred while running the epilogue, it
                // should not fail now. If it somehow fails here, there is no choice but to
                // discard the transaction.
                if let Err(e) = self.0.run_failure_epilogue(
                    &mut session,
                    gas_meter.balance(),
                    txn_data,
                    log_context,
                ) {
                    return discard_error_vm_status(e);
                }
                let txn_output =
                    get_transaction_output(&mut (), session, gas_meter.balance(), txn_data, status)
                        .unwrap_or_else(|e| discard_error_vm_status(e).1);
                (error_code, txn_output)
            }
            TransactionStatus::Discard(status) => {
                (VMStatus::Error(status), discard_error_output(status))
            }
            TransactionStatus::Retry => unreachable!(),
        }
    }

    fn success_transaction_cleanup<S: MoveResolverExt>(
        &self,
        mut session: SessionExt<S>,
        gas_meter: &mut AptosGasMeter,
        txn_data: &TransactionMetadata,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt), VMStatus> {
        self.0
            .run_success_epilogue(&mut session, gas_meter.balance(), txn_data, log_context)?;

        Ok((
            VMStatus::Executed,
            get_transaction_output(
                &mut (),
                session,
                gas_meter.balance(),
                txn_data,
                ExecutionStatus::Success,
            )?,
        ))
    }

    fn execute_script_or_script_function<S: MoveResolverExt>(
        &self,
        mut session: SessionExt<S>,
        gas_meter: &mut AptosGasMeter,
        txn_data: &TransactionMetadata,
        payload: &TransactionPayload,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt), VMStatus> {
        fail_point!("move_adapter::execute_script_or_script_function", |_| {
            Err(VMStatus::Error(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        });

        // Run the execution logic
        {
            gas_meter
                .charge_intrinsic_gas_for_transaction(txn_data.transaction_size())
                .map_err(|e| e.into_vm_status())?;

            match payload {
                TransactionPayload::Script(script) => {
                    let mut senders = vec![txn_data.sender()];
                    senders.extend(txn_data.secondary_signers());
                    let loaded_func =
                        session.load_script(script.code(), script.ty_args().to_vec())?;
                    let args = transaction_arg_validation::validate_combine_signer_and_txn_args(
                        &session,
                        senders,
                        convert_txn_args(script.args()),
                        &loaded_func,
                    )?;
                    session.execute_script(
                        script.code(),
                        script.ty_args().to_vec(),
                        args,
                        gas_meter,
                    )
                }
                TransactionPayload::ScriptFunction(script_fn) => {
                    let mut senders = vec![txn_data.sender()];

                    senders.extend(txn_data.secondary_signers());

                    let function = session.load_function(
                        script_fn.module(),
                        script_fn.function(),
                        script_fn.ty_args(),
                    )?;
                    let args = transaction_arg_validation::validate_combine_signer_and_txn_args(
                        &session,
                        senders,
                        script_fn.args().to_vec(),
                        &function,
                    )?;
                    session.execute_entry_function(
                        script_fn.module(),
                        script_fn.function(),
                        script_fn.ty_args().to_vec(),
                        args,
                        gas_meter,
                    )
                }
                TransactionPayload::ModuleBundle(_) | TransactionPayload::WriteSet(_) => {
                    return Err(VMStatus::Error(StatusCode::UNREACHABLE));
                }
            }
            .map_err(|e| e.into_vm_status())?;

            self.resolve_pending_code_publish(&mut session, gas_meter)?;

            self.success_transaction_cleanup(session, gas_meter, txn_data, log_context)
        }
    }

    fn verify_module_bundle<S: MoveResolverExt>(
        session: &mut SessionExt<S>,
        module_bundle: &ModuleBundle,
    ) -> VMResult<()> {
        for module_blob in module_bundle.iter() {
            match CompiledModule::deserialize(module_blob.code()) {
                Ok(module) => {
                    // verify the module doesn't exist
                    if session
                        .get_data_store()
                        .load_module(&module.self_id())
                        .is_ok()
                    {
                        return Err(verification_error(
                            StatusCode::DUPLICATE_MODULE_NAME,
                            IndexKind::AddressIdentifier,
                            module.self_handle_idx().0,
                        )
                        .finish(Location::Undefined));
                    }
                }
                Err(err) => return Err(err.finish(Location::Undefined)),
            }
        }
        Ok(())
    }

    /// Execute all module initializers.
    fn execute_module_initialization<S: MoveResolverExt>(
        &self,
        session: &mut SessionExt<S>,
        gas_meter: &mut AptosGasMeter,
        modules: &[CompiledModule],
        senders: &[AccountAddress],
    ) -> VMResult<()> {
        let init_func_name = ident_str!("init_module");
        for module in modules {
            let init_function = session.load_function(&module.self_id(), init_func_name, &[]);
            // it is ok to not have init_module function
            // init_module function should be (1) private and (2) has no return value
            if init_function.is_ok() {
                if verify_module_init_function(module).is_ok() {
                    let args: Vec<Vec<u8>> = senders
                        .iter()
                        .map(|s| MoveValue::Signer(*s).simple_serialize().unwrap())
                        .collect();
                    session.execute_function_bypass_visibility(
                        &module.self_id(),
                        init_func_name,
                        vec![],
                        args,
                        gas_meter,
                    )?;
                } else {
                    return Err(PartialVMError::new(StatusCode::VERIFICATION_ERROR)
                        .finish(Location::Undefined));
                }
            }
        }
        Ok(())
    }

    /// Deserialize a module bundle.
    fn deserialize_module_bundle(&self, modules: &ModuleBundle) -> VMResult<Vec<CompiledModule>> {
        let mut result = vec![];
        for module_blob in modules.iter() {
            match CompiledModule::deserialize(module_blob.code()) {
                Ok(module) => {
                    result.push(module);
                }
                Err(_err) => {
                    return Err(PartialVMError::new(StatusCode::CODE_DESERIALIZATION_ERROR)
                        .finish(Location::Undefined))
                }
            }
        }
        Ok(result)
    }

    /// Execute a module bundle load request.
    /// TODO: this is going to be deprecated and removed in favor of code publishing via
    /// NativeCodeContext
    fn execute_modules<S: MoveResolverExt>(
        &self,
        mut session: SessionExt<S>,
        gas_meter: &mut AptosGasMeter,
        txn_data: &TransactionMetadata,
        modules: &ModuleBundle,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt), VMStatus> {
        fail_point!("move_adapter::execute_module", |_| {
            Err(VMStatus::Error(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        });

        gas_meter
            .charge_intrinsic_gas_for_transaction(txn_data.transaction_size())
            .map_err(|e| e.into_vm_status())?;

        Self::verify_module_bundle(&mut session, modules)?;
        session
            .publish_module_bundle(modules.clone().into_inner(), txn_data.sender(), gas_meter)
            .map_err(|e| e.into_vm_status())?;

        // call init function of the each module
        self.execute_module_initialization(
            &mut session,
            gas_meter,
            &self.deserialize_module_bundle(modules)?,
            &[txn_data.sender()],
        )?;

        self.success_transaction_cleanup(session, gas_meter, txn_data, log_context)
    }

    /// Resolve a pending code publish request registered via the NativeCodeContext.
    fn resolve_pending_code_publish<S: MoveResolverExt>(
        &self,
        session: &mut SessionExt<S>,
        gas_meter: &mut AptosGasMeter,
    ) -> VMResult<()> {
        if let Some(PublishRequest {
            destination,
            bundle,
            expected_modules,
            check_compat,
        }) = session.extract_publish_request()
        {
            // TODO: unfortunately we need to deserialize the entire bundle here to handle
            // `init_module` and verify some deployment conditions, while the VM need to do
            // the deserialization again. Consider adding an API to MoveVM which allows to
            // directly pass CompiledModule.
            let modules = self.deserialize_module_bundle(&bundle)?;

            // Validate the module bundle
            self.validate_publish_request(&modules, expected_modules)?;

            // Publish the bundle
            if check_compat {
                session.publish_module_bundle(bundle.into_inner(), destination, gas_meter)?
            } else {
                session.publish_module_bundle_relax_compatibility(
                    bundle.into_inner(),
                    destination,
                    gas_meter,
                )?
            }

            // Execute initializers
            self.execute_module_initialization(session, gas_meter, &modules, &[destination])
        } else {
            Ok(())
        }
    }

    /// Validate a publish request.
    fn validate_publish_request(
        &self,
        modules: &[CompiledModule],
        expected_names: BTreeSet<String>,
    ) -> VMResult<()> {
        let given_names = modules
            .iter()
            .map(|m| m.self_id().name().as_str().to_string())
            .collect::<BTreeSet<_>>();
        if given_names != expected_names {
            Err(PartialVMError::new(StatusCode::VERIFICATION_ERROR)
                .with_message("metadata and code bundle mismatch".to_owned())
                .finish(Location::Undefined))
        } else {
            Ok(())
        }
    }

    pub(crate) fn execute_user_transaction<S: MoveResolverExt>(
        &self,
        storage: &S,
        txn: &SignatureCheckedTransaction,
        log_context: &AdapterLogSchema,
    ) -> (VMStatus, TransactionOutputExt) {
        macro_rules! unwrap_or_discard {
            ($res: expr) => {
                match $res {
                    Ok(s) => s,
                    Err(e) => return discard_error_vm_status(e),
                }
            };
        }

        // Revalidate the transaction.
        let mut session = self.0.new_session(storage, SessionId::txn(txn));
        if let Err(err) = validate_signature_checked_transaction::<S, Self>(
            self,
            &mut session,
            txn,
            false,
            log_context,
        ) {
            return discard_error_vm_status(err);
        };

        let gas_params = unwrap_or_discard!(self.0.get_gas_parameters(log_context));
        let txn_data = TransactionMetadata::new(txn);
        let mut gas_meter = AptosGasMeter::new(gas_params.clone(), txn_data.max_gas_amount());

        let result = match txn.payload() {
            payload @ TransactionPayload::Script(_)
            | payload @ TransactionPayload::ScriptFunction(_) => self
                .execute_script_or_script_function(
                    session,
                    &mut gas_meter,
                    &txn_data,
                    payload,
                    log_context,
                ),
            TransactionPayload::ModuleBundle(m) => {
                self.execute_modules(session, &mut gas_meter, &txn_data, m, log_context)
            }
            TransactionPayload::WriteSet(_) => {
                return discard_error_vm_status(VMStatus::Error(StatusCode::UNREACHABLE));
            }
        };

        let gas_usage = txn_data.max_gas_amount() - gas_meter.balance();
        TXN_GAS_USAGE.observe(gas_usage as f64);

        match result {
            Ok(output) => output,
            Err(err) => {
                let txn_status = TransactionStatus::from(err.clone());
                if txn_status.is_discarded() {
                    discard_error_vm_status(err)
                } else {
                    self.failed_transaction_cleanup_and_keep_vm_status(
                        err,
                        &mut gas_meter,
                        &txn_data,
                        storage,
                        log_context,
                    )
                }
            }
        }
    }

    fn execute_writeset<S: MoveResolverExt>(
        &self,
        storage: &S,
        writeset_payload: &WriteSetPayload,
        txn_sender: Option<AccountAddress>,
        session_id: SessionId,
    ) -> Result<ChangeSet, Result<(VMStatus, TransactionOutputExt), VMStatus>> {
        let mut gas_meter = UnmeteredGasMeter;

        Ok(match writeset_payload {
            WriteSetPayload::Direct(change_set) => change_set.clone(),
            WriteSetPayload::Script { script, execute_as } => {
                let mut tmp_session = self.0.new_session(storage, session_id);
                let senders = match txn_sender {
                    None => vec![*execute_as],
                    Some(sender) => vec![sender, *execute_as],
                };

                let loaded_func = tmp_session
                    .load_script(script.code(), script.ty_args().to_vec())
                    .map_err(|e| Err(e.into_vm_status()))?;
                let args = transaction_arg_validation::validate_combine_signer_and_txn_args(
                    &tmp_session,
                    senders,
                    convert_txn_args(script.args()),
                    &loaded_func,
                )
                .map_err(Err)?;

                let execution_result = tmp_session
                    .execute_script(
                        script.code(),
                        script.ty_args().to_vec(),
                        args,
                        &mut gas_meter,
                    )
                    .and_then(|_| tmp_session.finish())
                    .map_err(|e| e.into_vm_status());

                match execution_result {
                    Ok(session_out) => session_out.into_change_set(&mut ()).map_err(Err)?,
                    Err(e) => {
                        return Err(Ok((e, discard_error_output(StatusCode::INVALID_WRITE_SET))));
                    }
                }
            }
        })
    }

    fn read_writeset(
        &self,
        state_view: &impl StateView,
        write_set: &WriteSet,
    ) -> Result<(), VMStatus> {
        // All Move executions satisfy the read-before-write property. Thus we need to read each
        // access path that the write set is going to update.
        for (state_key, _) in write_set.iter() {
            state_view
                .get_state_value(state_key)
                .map_err(|_| VMStatus::Error(StatusCode::STORAGE_ERROR))?;
        }
        Ok(())
    }

    fn validate_waypoint_change_set(
        change_set: &ChangeSet,
        log_context: &AdapterLogSchema,
    ) -> Result<(), VMStatus> {
        let has_new_block_event = change_set
            .events()
            .iter()
            .any(|e| *e.key() == new_block_event_key());
        let has_new_epoch_event = change_set
            .events()
            .iter()
            .any(|e| *e.key() == new_epoch_event_key());
        if has_new_block_event && has_new_epoch_event {
            Ok(())
        } else {
            error!(
                *log_context,
                "[aptos_vm] waypoint txn needs to emit new epoch and block"
            );
            Err(VMStatus::Error(StatusCode::INVALID_WRITE_SET))
        }
    }

    pub(crate) fn process_waypoint_change_set<S: MoveResolverExt + StateView>(
        &self,
        storage: &S,
        writeset_payload: WriteSetPayload,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt), VMStatus> {
        // TODO: user specified genesis id to distinguish different genesis write sets
        let genesis_id = HashValue::zero();
        let change_set = match self.execute_writeset(
            storage,
            &writeset_payload,
            None,
            SessionId::genesis(genesis_id),
        ) {
            Ok(cs) => cs,
            Err(e) => return e,
        };
        Self::validate_waypoint_change_set(&change_set, log_context)?;
        let (write_set, events) = change_set.into_inner();
        self.read_writeset(storage, &write_set)?;
        SYSTEM_TRANSACTIONS_EXECUTED.inc();
        Ok((
            VMStatus::Executed,
            TransactionOutputExt::from(TransactionOutput::new(
                write_set,
                events,
                0,
                VMStatus::Executed.into(),
            )),
        ))
    }

    pub(crate) fn process_block_prologue<S: MoveResolverExt>(
        &self,
        storage: &S,
        block_metadata: BlockMetadata,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt), VMStatus> {
        fail_point!("move_adapter::process_block_prologue", |_| {
            Err(VMStatus::Error(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        });

        let txn_data = TransactionMetadata {
            sender: account_config::reserved_vm_address(),
            max_gas_amount: 0,
            ..Default::default()
        };
        let mut gas_meter = UnmeteredGasMeter;
        let mut session = self
            .0
            .new_session(storage, SessionId::block_meta(&block_metadata));

        let args = serialize_values(&block_metadata.get_prologue_move_args(txn_data.sender));
        session
            .execute_function_bypass_visibility(
                &BLOCK_MODULE,
                BLOCK_PROLOGUE,
                vec![],
                args,
                &mut gas_meter,
            )
            .map(|_return_vals| ())
            .or_else(|e| {
                expect_only_successful_execution(e, BLOCK_PROLOGUE.as_str(), log_context)
            })?;
        SYSTEM_TRANSACTIONS_EXECUTED.inc();

        let output =
            get_transaction_output(&mut (), session, 0, &txn_data, ExecutionStatus::Success)?;
        Ok((VMStatus::Executed, output))
    }

    pub(crate) fn process_writeset_transaction<S: MoveResolverExt + StateView>(
        &self,
        storage: &S,
        txn: &SignatureCheckedTransaction,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt), VMStatus> {
        fail_point!("move_adapter::process_writeset_transaction", |_| {
            Err(VMStatus::Error(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        });

        // Revalidate the transaction.
        let mut session = self.0.new_session(storage, SessionId::txn(txn));
        if let Err(e) = validate_signature_checked_transaction::<S, Self>(
            self,
            &mut session,
            txn,
            false,
            log_context,
        ) {
            return Ok(discard_error_vm_status(e));
        };
        self.execute_writeset_transaction(
            storage,
            match txn.payload() {
                TransactionPayload::WriteSet(writeset_payload) => writeset_payload,
                TransactionPayload::ModuleBundle(_)
                | TransactionPayload::Script(_)
                | TransactionPayload::ScriptFunction(_) => {
                    log_context.alert();
                    error!(*log_context, "[aptos_vm] UNREACHABLE");
                    return Ok(discard_error_vm_status(VMStatus::Error(
                        StatusCode::UNREACHABLE,
                    )));
                }
            },
            TransactionMetadata::new(txn),
            log_context,
        )
    }

    pub fn execute_writeset_transaction<S: MoveResolverExt + StateView>(
        &self,
        storage: &S,
        writeset_payload: &WriteSetPayload,
        txn_data: TransactionMetadata,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt), VMStatus> {
        let change_set = match self.execute_writeset(
            storage,
            writeset_payload,
            Some(txn_data.sender()),
            SessionId::txn_meta(&txn_data),
        ) {
            Ok(change_set) => change_set,
            Err(e) => return e,
        };

        // Run the epilogue function.
        let mut session = self.0.new_session(storage, SessionId::txn_meta(&txn_data));
        self.0.run_writeset_epilogue(
            &mut session,
            &txn_data,
            writeset_payload.should_trigger_reconfiguration_by_default(),
            log_context,
        )?;

        if let Err(e) = self.read_writeset(storage, change_set.write_set()) {
            // Any error at this point would be an invalid writeset
            return Ok((e, discard_error_output(StatusCode::INVALID_WRITE_SET)));
        };

        let session_out = session.finish().map_err(|e| e.into_vm_status())?;
        let (epilogue_writeset, epilogue_events) =
            session_out.into_change_set(&mut ())?.into_inner();

        // Make sure epilogue WriteSet doesn't intersect with the writeset in TransactionPayload.
        if !epilogue_writeset
            .iter()
            .map(|(ap, _)| ap)
            .collect::<HashSet<_>>()
            .is_disjoint(
                &change_set
                    .write_set()
                    .iter()
                    .map(|(ap, _)| ap)
                    .collect::<HashSet<_>>(),
            )
        {
            let vm_status = VMStatus::Error(StatusCode::INVALID_WRITE_SET);
            return Ok(discard_error_vm_status(vm_status));
        }
        if !epilogue_events
            .iter()
            .map(|event| event.key())
            .collect::<HashSet<_>>()
            .is_disjoint(
                &change_set
                    .events()
                    .iter()
                    .map(|event| event.key())
                    .collect::<HashSet<_>>(),
            )
        {
            let vm_status = VMStatus::Error(StatusCode::INVALID_WRITE_SET);
            return Ok(discard_error_vm_status(vm_status));
        }

        let write_set = WriteSetMut::new(
            epilogue_writeset
                .iter()
                .chain(change_set.write_set().iter())
                .cloned()
                .collect(),
        )
        .freeze()
        .map_err(|_| VMStatus::Error(StatusCode::INVALID_WRITE_SET))?;
        let events = change_set
            .events()
            .iter()
            .chain(epilogue_events.iter())
            .cloned()
            .collect();
        SYSTEM_TRANSACTIONS_EXECUTED.inc();

        Ok((
            VMStatus::Executed,
            TransactionOutputExt::from(TransactionOutput::new(
                write_set,
                events,
                0,
                TransactionStatus::Keep(ExecutionStatus::Success),
            )),
        ))
    }

    /// Alternate form of 'execute_block' that keeps the vm_status before it goes into the
    /// `TransactionOutput`
    pub fn execute_block_and_keep_vm_status(
        transactions: Vec<Transaction>,
        state_view: &impl StateView,
    ) -> Result<Vec<(VMStatus, TransactionOutput)>, VMStatus> {
        let mut state_view_cache = StateViewCache::new(state_view);
        let count = transactions.len();
        let vm = AptosVM::new(&state_view_cache);
        let res = adapter_common::execute_block_impl(&vm, transactions, &mut state_view_cache)?;
        // Record the histogram count for transactions per block.
        BLOCK_TRANSACTION_COUNT.observe(count as f64);
        Ok(res)
    }

    pub fn simulate_signed_transaction(
        txn: &SignedTransaction,
        state_view: &impl StateView,
    ) -> (VMStatus, TransactionOutputExt) {
        let vm = AptosVM::new(state_view);
        let simulation_vm = AptosSimulationVM(vm);
        let log_context = AdapterLogSchema::new(state_view.id(), 0);
        simulation_vm.simulate_signed_transaction(&state_view.as_move_resolver(), txn, &log_context)
    }

    fn run_prologue_with_payload<S: MoveResolverExt>(
        &self,
        session: &mut SessionExt<S>,
        payload: &TransactionPayload,
        txn_data: &TransactionMetadata,
        log_context: &AdapterLogSchema,
    ) -> Result<(), VMStatus> {
        match payload {
            TransactionPayload::Script(_) => {
                self.0.check_gas(txn_data, log_context)?;
                self.0.run_script_prologue(session, txn_data, log_context)
            }
            TransactionPayload::ScriptFunction(_) => {
                // NOTE: Script and ScriptFunction shares the same prologue
                self.0.check_gas(txn_data, log_context)?;
                self.0.run_script_prologue(session, txn_data, log_context)
            }
            TransactionPayload::ModuleBundle(_module) => {
                self.0.check_gas(txn_data, log_context)?;
                self.0.run_module_prologue(session, txn_data, log_context)
            }
            TransactionPayload::WriteSet(_cs) => {
                self.0.run_writeset_prologue(session, txn_data, log_context)
            }
        }
    }
}

// Executor external API
impl VMExecutor for AptosVM {
    /// Execute a block of `transactions`. The output vector will have the exact same length as the
    /// input vector. The discarded transactions will be marked as `TransactionStatus::Discard` and
    /// have an empty `WriteSet`. Also `state_view` is immutable, and does not have interior
    /// mutability. Writes to be applied to the data view are encoded in the write set part of a
    /// transaction output.
    fn execute_block(
        transactions: Vec<Transaction>,
        state_view: &impl StateView,
    ) -> Result<Vec<TransactionOutput>, VMStatus> {
        fail_point!("move_adapter::execute_block", |_| {
            Err(VMStatus::Error(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        });

        let concurrency_level = Self::get_concurrency_level();
        if concurrency_level > 1 {
            let (result, err) = crate::parallel_executor::ParallelAptosVM::execute_block(
                transactions,
                state_view,
                concurrency_level,
            )?;
            debug!("Parallel execution error {:?}", err);
            Ok(result)
        } else {
            let output = Self::execute_block_and_keep_vm_status(transactions, state_view)?;
            Ok(output
                .into_iter()
                .map(|(_vm_status, txn_output)| txn_output)
                .collect())
        }
    }
}

// VMValidator external API
impl VMValidator for AptosVM {
    /// Determine if a transaction is valid. Will return `None` if the transaction is accepted,
    /// `Some(Err)` if the VM rejects it, with `Err` as an error code. Verification performs the
    /// following steps:
    /// 1. The signature on the `SignedTransaction` matches the public key included in the
    ///    transaction
    /// 2. The script to be executed is under given specific configuration.
    /// 3. Invokes `Account.prologue`, which checks properties such as the transaction has the
    /// right sequence number and the sender has enough balance to pay for the gas.
    /// TBD:
    /// 1. Transaction arguments matches the main function's type signature.
    ///    We don't check this item for now and would execute the check at execution time.
    fn validate_transaction(
        &self,
        transaction: SignedTransaction,
        state_view: &impl StateView,
    ) -> VMValidatorResult {
        validate_signed_transaction(self, transaction, state_view)
    }
}

impl VMAdapter for AptosVM {
    fn new_session<'r, R: MoveResolverExt>(
        &self,
        remote: &'r R,
        session_id: SessionId,
    ) -> SessionExt<'r, '_, R> {
        self.0.new_session(remote, session_id)
    }

    fn check_signature(txn: SignedTransaction) -> Result<SignatureCheckedTransaction> {
        txn.check_signature()
    }

    fn check_transaction_format(&self, txn: &SignedTransaction) -> Result<(), VMStatus> {
        if txn.contains_duplicate_signers() {
            return Err(VMStatus::Error(StatusCode::SIGNERS_CONTAIN_DUPLICATES));
        }

        Ok(())
    }

    fn run_prologue<S: MoveResolverExt>(
        &self,
        session: &mut SessionExt<S>,
        transaction: &SignatureCheckedTransaction,
        log_context: &AdapterLogSchema,
    ) -> Result<(), VMStatus> {
        let txn_data = TransactionMetadata::new(transaction);
        //let account_blob = session.data_cache.get_resource
        self.run_prologue_with_payload(session, transaction.payload(), &txn_data, log_context)
    }

    fn should_restart_execution(vm_output: &TransactionOutput) -> bool {
        let new_epoch_event_key = aptos_types::on_chain_config::new_epoch_event_key();
        vm_output
            .events()
            .iter()
            .any(|event| *event.key() == new_epoch_event_key)
    }

    fn execute_single_transaction<S: MoveResolverExt + StateView>(
        &self,
        txn: &PreprocessedTransaction,
        data_cache: &S,
        log_context: &AdapterLogSchema,
    ) -> Result<(VMStatus, TransactionOutputExt, Option<String>), VMStatus> {
        Ok(match txn {
            PreprocessedTransaction::BlockMetadata(block_metadata) => {
                let (vm_status, output) =
                    self.process_block_prologue(data_cache, block_metadata.clone(), log_context)?;
                (vm_status, output, Some("block_prologue".to_string()))
            }
            PreprocessedTransaction::WaypointWriteSet(write_set_payload) => {
                let (vm_status, output) = self.process_waypoint_change_set(
                    data_cache,
                    write_set_payload.clone(),
                    log_context,
                )?;
                (vm_status, output, Some("waypoint_write_set".to_string()))
            }
            PreprocessedTransaction::UserTransaction(txn) => {
                let sender = txn.sender().to_string();
                let _timer = TXN_TOTAL_SECONDS.start_timer();
                let (vm_status, output) =
                    self.execute_user_transaction(data_cache, txn, log_context);

                // Increment the counter for user transactions executed.
                let counter_label = match output.status() {
                    TransactionStatus::Keep(_) => Some("success"),
                    TransactionStatus::Discard(_) => Some("discarded"),
                    TransactionStatus::Retry => None,
                };
                if let Some(label) = counter_label {
                    USER_TRANSACTIONS_EXECUTED.with_label_values(&[label]).inc();
                }
                (vm_status, output, Some(sender))
            }
            PreprocessedTransaction::WriteSet(txn) => {
                let (vm_status, output) =
                    self.process_writeset_transaction(data_cache, txn, log_context)?;
                (vm_status, output, Some("write_set".to_string()))
            }
            PreprocessedTransaction::InvalidSignature => {
                let (vm_status, output) =
                    discard_error_vm_status(VMStatus::Error(StatusCode::INVALID_SIGNATURE));
                (vm_status, output, None)
            }
            PreprocessedTransaction::StateCheckpoint => {
                let output = TransactionOutput::new(
                    WriteSet::default(),
                    Vec::new(),
                    0,
                    TransactionStatus::Keep(ExecutionStatus::Success),
                );
                (
                    VMStatus::Executed,
                    TransactionOutputExt::from(output),
                    Some("state_checkpoint".into()),
                )
            }
        })
    }
}

impl AsRef<AptosVMImpl> for AptosVM {
    fn as_ref(&self) -> &AptosVMImpl {
        &self.0
    }
}

impl AsMut<AptosVMImpl> for AptosVM {
    fn as_mut(&mut self) -> &mut AptosVMImpl {
        &mut self.0
    }
}

impl AptosSimulationVM {
    fn validate_simulated_transaction<S: MoveResolverExt>(
        &self,
        session: &mut SessionExt<S>,
        transaction: &SignedTransaction,
        txn_data: &TransactionMetadata,
        log_context: &AdapterLogSchema,
    ) -> Result<(), VMStatus> {
        self.0.check_transaction_format(transaction)?;
        self.0
            .run_prologue_with_payload(session, transaction.payload(), txn_data, log_context)
    }

    /*
    Executes a SignedTransaction without performing signature verification
     */
    fn simulate_signed_transaction<S: MoveResolverExt>(
        &self,
        storage: &S,
        txn: &SignedTransaction,
        log_context: &AdapterLogSchema,
    ) -> (VMStatus, TransactionOutputExt) {
        // simulation transactions should not carry valid signatures, otherwise malicious fullnodes
        // may execute them without user's explicit permission.
        if txn.clone().check_signature().is_ok() {
            return discard_error_vm_status(VMStatus::Error(StatusCode::INVALID_SIGNATURE));
        }

        // Revalidate the transaction.
        let txn_data = TransactionMetadata::new(txn);
        let mut session = self.0.new_session(storage, SessionId::txn_meta(&txn_data));
        if let Err(err) =
            self.validate_simulated_transaction::<S>(&mut session, txn, &txn_data, log_context)
        {
            return discard_error_vm_status(err);
        };

        let gas_params = match self.0 .0.get_gas_parameters(log_context) {
            Err(err) => return discard_error_vm_status(err),
            Ok(s) => s,
        };
        let mut gas_meter = AptosGasMeter::new(gas_params.clone(), txn_data.max_gas_amount());

        let result = match txn.payload() {
            payload @ TransactionPayload::Script(_)
            | payload @ TransactionPayload::ScriptFunction(_) => {
                self.0.execute_script_or_script_function(
                    session,
                    &mut gas_meter,
                    &txn_data,
                    payload,
                    log_context,
                )
            }
            TransactionPayload::ModuleBundle(m) => {
                self.0
                    .execute_modules(session, &mut gas_meter, &txn_data, m, log_context)
            }
            TransactionPayload::WriteSet(_) => {
                return discard_error_vm_status(VMStatus::Error(StatusCode::UNREACHABLE));
            }
        };

        match result {
            Ok(output) => output,
            Err(err) => {
                let txn_status = TransactionStatus::from(err.clone());
                if txn_status.is_discarded() {
                    discard_error_vm_status(err)
                } else {
                    let (vm_status, output) = self.0.failed_transaction_cleanup_and_keep_vm_status(
                        err,
                        &mut gas_meter,
                        &txn_data,
                        storage,
                        log_context,
                    );
                    (vm_status, output)
                }
            }
        }
    }
}
