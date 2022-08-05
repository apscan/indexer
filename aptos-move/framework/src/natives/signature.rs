// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_crypto::{bls12381, ed25519, traits::*};
use curve25519_dalek::edwards::CompressedEdwardsY;
use move_deps::move_vm_types::values::Struct;
use move_deps::{
    move_binary_format::errors::PartialVMResult,
    move_core_types::gas_schedule::GasCost,
    move_vm_runtime::native_functions::NativeContext,
    move_vm_types::{
        gas_schedule::NativeCostIndex,
        loaded_data::runtime_types::Type,
        natives::function::{native_gas, NativeResult},
        pop_arg,
        values::Value,
    },
};
use smallvec::smallvec;
use std::{collections::VecDeque, convert::TryFrom};

/// Returns the equivalent of a Move std::option::none() natively in Rust.
/// TODO: vector_for_testing_only is not an API we conceptually support and misusing it could cause the VM to crash.
fn none_option() -> Value {
    Value::struct_(Struct::pack(std::iter::once(
        Value::vector_for_testing_only(std::iter::empty()),
    )))
}

/// Returns the equivalent of a Move std::option<vector<u8>>::some(v) natively in Rust.
/// TODO: vector_for_testing_only is not an API we conceptually support and misusing it could cause the VM to crash.
fn some_option(v: Vec<u8>) -> Value {
    let vv = Value::vector_u8(v.into_iter());
    Value::struct_(Struct::pack(std::iter::once(
        Value::vector_for_testing_only(std::iter::once(vv)),
    )))
}

/// Used to pop a Vec<Vec<u8>> argument off the stack.
macro_rules! pop_vec_arg {
    ($arguments:ident, $t:ty) => {{
        // Replicating the code from pop_arg! here
        use move_deps::move_vm_types::natives::function::{PartialVMError, StatusCode};
        let value_vec = match $arguments.pop_back().map(|v| v.value_as::<Vec<Value>>()) {
            None => {
                return Err(PartialVMError::new(
                    StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
                ))
            }
            Some(Err(e)) => return Err(e),
            Some(Ok(v)) => v,
        };

        // Pop each Value from the popped Vec<Value>, cast it as a Vec<u8>, and push it to a Vec<Vec<u8>>
        let mut vec_vec = vec![];
        for value in value_vec {
            let vec = match value.value_as::<$t>() {
                Err(e) => return Err(e),
                Ok(v) => v,
            };
            vec_vec.push(vec);
        }

        vec_vec
    }};
}

/// Deserializes a vector of PK bytes into bls12381::PublicKey structs.
fn bls12381_deserialize_pks_helper(pks_serialized: Vec<Vec<u8>>) -> Vec<bls12381::PublicKey> {
    let mut pks = vec![];

    for pk_bytes in pks_serialized {
        // NOTE(Gas): O(1) deserialization cost
        let pk = match bls12381::PublicKey::try_from(&pk_bytes[..]) {
            Ok(key) => key,
            // If PK does not deserialize correctly, break early
            Err(_) => break,
        };

        pks.push(pk);
    }

    pks
}

/// This is a helper function called by our many `bls12381_verify_*` functions
pub fn bls12381_verify_signature_helper(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
    check_pk_subgroup: bool,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    // TODO(Gas): replace with proper gas cost
    let cost = GasCost::new(super::cost::APTOS_LIB_TYPE_OF, 1).total();

    let msg_bytes = pop_arg!(arguments, Vec<u8>);
    let aggpk_bytes = pop_arg!(arguments, Vec<u8>);
    let multisig_bytes = pop_arg!(arguments, Vec<u8>);

    // NOTE(Gas): O(1) deserialization cost
    let pk = match bls12381::PublicKey::try_from(&aggpk_bytes[..]) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
        }
    };

    // NOTE(Gas): O(1) cost (around 39 microseconds on Apple M1)
    if check_pk_subgroup && pk.subgroup_check().is_err() {
        return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
    }

    // NOTE(Gas): O(1) deserialization cost
    let sig = match bls12381::Signature::try_from(&multisig_bytes[..]) {
        Ok(sig) => sig,
        Err(_) => {
            return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
        }
    };

    // NOTE(Gas): O(1) cost: 2 bilinear pairings and a hash-to-curve
    let verify_result = sig.verify_arbitrary_msg(&msg_bytes[..], &pk).is_ok();

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::bool(verify_result)],
    ))
}

pub fn native_bls12381_aggregate_pop_verified_pubkeys(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    // TODO(Gas): replace with proper gas cost
    let cost = GasCost::new(super::cost::APTOS_LIB_TYPE_OF, 1).total();

    // Parses a Vec<Vec<u8>> of all serialized public keys
    let pks_serialized = pop_vec_arg!(arguments, Vec<u8>);
    let num_pks = pks_serialized.len();

    // NOTE(Gas): The gas cost will be proportional to |pks|
    let pks = bls12381_deserialize_pks_helper(pks_serialized);

    // If not all PKs were successfully deserialized, return None.
    if pks.len() != num_pks {
        return Ok(NativeResult::ok(cost, smallvec![none_option()]));
    }

    // Aggregate the public keys (this will NOT group-check the individual PKs)
    let aggpk =
        // NOTE(Gas): O(|pks|) cost: |pks| elliptic curve additions
        match bls12381::PublicKey::aggregate(pks.iter().collect::<Vec<&bls12381::PublicKey>>()) {
            Ok(aggpk) => aggpk,
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![none_option()])),
        };

    Ok(NativeResult::ok(
        cost,
        smallvec![some_option(aggpk.to_bytes().to_vec())],
    ))
}

pub fn native_bls12381_aggregate_signatures(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // TODO(Gas): replace with proper gas cost
    let cost = GasCost::new(super::cost::APTOS_LIB_TYPE_OF, 1).total();

    // Parses a Vec<Vec<u8>> of all serialized signatures
    let sigs_serialized = pop_vec_arg!(arguments, Vec<u8>);
    let mut sigs = vec![];

    for sig_bytes in sigs_serialized {
        // NOTE(Gas): O(1) deserialization cost
        let sig = match bls12381::Signature::try_from(&sig_bytes[..]) {
            Ok(sig) => sig,
            // If signature does not deserialize correctly, return None.
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![none_option()])),
        };

        sigs.push(sig);
    }

    // If zero signatures were given as input, return None.
    if sigs.is_empty() {
        return Ok(NativeResult::ok(cost, smallvec![none_option()]));
    }

    // Aggregate the signatures (this will NOT group-check the individual signatures)
    let aggsig =
        // NOTE(Gas): O(|sigs|) cost: |sigs| elliptic curve additions
        match bls12381::Signature::aggregate(sigs) {
            Ok(aggsig) => aggsig,
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![none_option()])),
        };

    Ok(NativeResult::ok(
        cost,
        smallvec![some_option(aggsig.to_bytes().to_vec())],
    ))
}

pub fn native_bls12381_signature_subgroup_check(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    // TODO(Gas): replace with proper gas cost
    let cost = GasCost::new(super::cost::APTOS_LIB_TYPE_OF, 1).total();

    let sig_bytes = pop_arg!(arguments, Vec<u8>);

    // NOTE(Gas): O(1) deserialization cost
    let sig = match bls12381::Signature::try_from(&sig_bytes[..]) {
        Ok(key) => key,
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    // NOTE(Gas): O(1) cost: uses endomorphisms for performing faster subgroup checks
    let valid = sig.subgroup_check().is_ok();

    Ok(NativeResult::ok(cost, smallvec![Value::bool(valid)]))
}

pub fn native_bls12381_validate_pubkey(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    // TODO(Gas): replace with proper gas cost
    let cost = GasCost::new(super::cost::APTOS_LIB_TYPE_OF, 1).total();

    let pk_bytes = pop_arg!(arguments, Vec<u8>);

    // NOTE(Gas): O(1) deserialization cost
    let public_key = match bls12381::PublicKey::try_from(&pk_bytes[..]) {
        Ok(key) => key,
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    // NOTE(Gas): O(1) cost: uses endomorphisms for performing faster subgroup checks
    let valid = public_key.subgroup_check().is_ok();

    Ok(NativeResult::ok(cost, smallvec![Value::bool(valid)]))
}

pub fn native_bls12381_verify_proof_of_possession(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    // TODO(Gas): replace with proper gas cost
    let cost = GasCost::new(super::cost::APTOS_LIB_TYPE_OF, 1).total();

    let pop_bytes = pop_arg!(arguments, Vec<u8>);
    let key_bytes = pop_arg!(arguments, Vec<u8>);

    // NOTE(Gas): O(1) deserialization cost
    let pop = match bls12381::ProofOfPossession::try_from(&pop_bytes[..]) {
        Ok(pop) => pop,
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    // NOTE(Gas): O(1) deserialization cost
    let public_key = match bls12381::PublicKey::try_from(&key_bytes[..]) {
        Ok(key) => key,
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    // NOTE(Gas): O(1) cost: 2 bilinear pairings and a hash-to-curve
    let valid = pop.verify(&public_key).is_ok();

    Ok(NativeResult::ok(cost, smallvec![Value::bool(valid)]))
}

pub fn native_bls12381_verify_aggregate_signature(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    // TODO(Gas): replace with proper gas cost
    let cost = GasCost::new(super::cost::APTOS_LIB_TYPE_OF, 1).total();

    // Parses a Vec<Vec<u8>> of all messages
    let messages = pop_vec_arg!(arguments, Vec<u8>);
    // Parses a Vec<Vec<u8>> of all serialized public keys
    let pks_serialized = pop_vec_arg!(arguments, Vec<u8>);
    let num_pks = pks_serialized.len();

    // Parses the signature as a Vec<u8>
    let aggsig_bytes = pop_arg!(arguments, Vec<u8>);

    // Number of messages must match number of public keys
    if pks_serialized.len() != messages.len() {
        return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
    }

    let pks = bls12381_deserialize_pks_helper(pks_serialized);

    // If less PKs than expected were deserialized, return None.
    if pks.len() != num_pks {
        return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
    }

    // NOTE(Gas): O(1) deserialization cost
    let aggsig = match bls12381::Signature::try_from(&aggsig_bytes[..]) {
        Ok(key) => key,
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    let msgs_refs = messages
        .iter()
        .map(|m| m.as_slice())
        .collect::<Vec<&[u8]>>();
    let pks_refs = pks.iter().collect::<Vec<&bls12381::PublicKey>>();
    let verify_result = aggsig
        .verify_aggregate_arbitrary_msg(&msgs_refs, &pks_refs)
        .is_ok();

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::bool(verify_result)],
    ))
}

pub fn native_bls12381_verify_multisignature(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let check_pk_subgroup = false;
    bls12381_verify_signature_helper(_context, _ty_args, arguments, check_pk_subgroup)
}

pub fn native_bls12381_verify_normal_signature(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // For normal (non-aggregated) signatures, PK's typically don't come with PoPs and the caller
    // might forget to check prime-order subgroup membership of the PK. Therefore, we always enforce
    // it here.
    let check_pk_subgroup = true;
    bls12381_verify_signature_helper(_context, _ty_args, arguments, check_pk_subgroup)
}

pub fn native_bls12381_verify_signature_share(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // For signature shares, the caller is REQUIRED to check the PK's PoP, and thus the PK is in the
    // prime-order subgroup.
    let check_pk_subgroup = false;
    bls12381_verify_signature_helper(_context, _ty_args, arguments, check_pk_subgroup)
}

pub fn native_ed25519_validate_pubkey(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let key_bytes = pop_arg!(arguments, Vec<u8>);

    let cost = native_gas(
        context.cost_table(),
        NativeCostIndex::ED25519_VALIDATE_KEY,
        key_bytes.len(),
    );

    // NOTE(Gas): O(1) deserialization cost
    let key_bytes_slice = match <[u8; 32]>::try_from(key_bytes) {
        Ok(slice) => slice,
        Err(_) => {
            return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
        }
    };

    // This deserialization only performs point-on-curve checks, so we check for small subgroup below
    // NOTE(Gas): O(1) cost: some arithmetic for converting to (X, Y, Z, T) coordinates
    let point = match CompressedEdwardsY(key_bytes_slice).decompress() {
        Some(point) => point,
        None => {
            return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
        }
    };

    // Check if the point lies on a small subgroup. This is required when using curves with a
    // small cofactor (e.g., in Ed25519, cofactor = 8).
    // NOTE(Gas): O(1) cost: multiplies the point by the cofactor
    let valid = !point.is_small_order();

    Ok(NativeResult::ok(cost, smallvec![Value::bool(valid)]))
}

pub fn native_ed25519_verify_signature(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let msg = pop_arg!(arguments, Vec<u8>);
    let pubkey = pop_arg!(arguments, Vec<u8>);
    let signature = pop_arg!(arguments, Vec<u8>);

    let cost = native_gas(
        context.cost_table(),
        NativeCostIndex::ED25519_VERIFY,
        msg.len(),
    );

    // NOTE(Gas): O(1) deserialization cost
    let sig = match ed25519::Ed25519Signature::try_from(signature.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
        }
    };

    // NOTE(Gas): O(1) deserialization cost
    let pk = match ed25519::Ed25519PublicKey::try_from(pubkey.as_slice()) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)]));
        }
    };

    // NOTE(Gas): O(1) cost: a size-2 multi-scalar multiplication
    let verify_result = sig.verify_arbitrary_msg(msg.as_slice(), &pk).is_ok();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::bool(verify_result)],
    ))
}

pub fn native_secp256k1_ecdsa_recover(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let signature = pop_arg!(arguments, Vec<u8>);
    let recovery_id = pop_arg!(arguments, u8);
    let msg = pop_arg!(arguments, Vec<u8>);

    let cost = GasCost::new(super::cost::APTOS_SECP256K1_RECOVER, 1).total();

    // NOTE(Gas): O(1) cost
    // (In reality, O(|msg|) deserialization cost, with |msg| < libsecp256k1_core::util::MESSAGE_SIZE
    // which seems to be 32 bytes, so O(1) cost for all intents and purposes.)
    let msg = match libsecp256k1::Message::parse_slice(&msg) {
        Ok(msg) => msg,
        Err(_) => {
            return Ok(NativeResult::ok(
                cost,
                smallvec![Value::vector_u8([0u8; 0]), Value::bool(false)],
            ));
        }
    };

    // NOTE(Gas): O(1) cost
    let rid = match libsecp256k1::RecoveryId::parse(recovery_id) {
        Ok(rid) => rid,
        Err(_) => {
            return Ok(NativeResult::ok(
                cost,
                smallvec![Value::vector_u8([0u8; 0]), Value::bool(false)],
            ));
        }
    };

    // NOTE(Gas): O(1) deserialization cost
    // which seems to be 64 bytes, so O(1) cost for all intents and purposes.
    let sig = match libsecp256k1::Signature::parse_standard_slice(&signature) {
        Ok(sig) => sig,
        Err(_) => {
            return Ok(NativeResult::ok(
                cost,
                smallvec![Value::vector_u8([0u8; 0]), Value::bool(false)],
            ));
        }
    };

    // NOTE(Gas): O(1) cost: a size-2 multi-scalar multiplication
    let pk = match libsecp256k1::recover(&msg, &sig, &rid) {
        Ok(pk) => pk,
        Err(_) => {
            return Ok(NativeResult::ok(
                cost,
                smallvec![Value::vector_u8([0u8; 0]), Value::bool(false)],
            ));
        }
    };

    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::vector_u8(pk.serialize()[1..].to_vec()),
            Value::bool(true)
        ],
    ))
}
