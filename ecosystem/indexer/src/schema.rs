table! {
    account_resources (address, hash) {
        address -> Varchar,
        hash -> Varchar,
        version -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

table! {
    block_metadata_transactions (version) {
        version -> Int8,
        id -> Varchar,
        round -> Int8,
        previous_block_votes -> Jsonb,
        proposer -> Varchar,
        timestamp -> Timestamp,
        inserted_at -> Timestamp,
        epoch -> Int8,
        previous_block_votes_bitmap -> Jsonb,
        failed_proposer_indices -> Jsonb,
    }
}

table! {
    blocks (height) {
        transaction_version -> Int8,
        epoch -> Int8,
        round -> Int8,
        height -> Int8,
        hash -> Nullable<Varchar>,
        time_microseconds -> Int8,
        previous_block_votes -> Jsonb,
        failed_proposer_indices -> Jsonb,
    }
}

table! {
    collections (collection_id) {
        collection_id -> Varchar,
        creator -> Varchar,
        name -> Varchar,
        description -> Varchar,
        max_amount -> Nullable<Varchar>,
        uri -> Varchar,
        created_at -> Timestamp,
        inserted_at -> Timestamp,
    }
}

table! {
    direct_writeset_payload (transaction_version) {
        transaction_version -> Int8,
        events -> Jsonb,
        changes -> Jsonb,
    }
}

table! {
    events (key, sequence_number) {
        transaction_version -> Int8,
        key -> Varchar,
        sequence_number -> Int8,
        #[sql_name = "type"]
        type_ -> Text,
        data -> Jsonb,
        inserted_at -> Timestamp,
    }
}

table! {
    ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

table! {
    metadatas (token_id) {
        token_id -> Varchar,
        name -> Nullable<Varchar>,
        symbol -> Nullable<Varchar>,
        seller_fee_basis_points -> Nullable<Int8>,
        description -> Nullable<Varchar>,
        image -> Varchar,
        external_url -> Nullable<Varchar>,
        animation_url -> Nullable<Varchar>,
        attributes -> Nullable<Jsonb>,
        properties -> Nullable<Jsonb>,
        last_updated_at -> Timestamp,
        inserted_at -> Timestamp,
    }
}

table! {
    module_bundle_payload (transaction_version) {
        transaction_version -> Int8,
        module_changes -> Jsonb,
    }
}

table! {
    module_changes (transaction_version, transaction_index) {
        transaction_version -> Int8,
        transaction_index -> Int4,
        is_write -> Bool,
        address -> Varchar,
        state_key_hash -> Varchar,
        move_module_address -> Varchar,
        move_module_name -> Varchar,
        move_module_bytecode -> Varchar,
        move_module_abi -> Jsonb,
    }
}

table! {
    ownerships (ownership_id) {
        ownership_id -> Varchar,
        token_id -> Nullable<Varchar>,
        owner -> Nullable<Varchar>,
        amount -> Int8,
        updated_at -> Timestamp,
        inserted_at -> Timestamp,
    }
}

table! {
    processor_statuses (name, version) {
        name -> Varchar,
        version -> Int8,
        success -> Bool,
        details -> Nullable<Text>,
        last_updated -> Timestamp,
    }
}

table! {
    resource_changes (transaction_version, transaction_index) {
        transaction_version -> Int8,
        transaction_index -> Int4,
        is_write -> Bool,
        address -> Varchar,
        state_key_hash -> Varchar,
        move_resource_address -> Varchar,
        move_resource_module -> Varchar,
        move_resource_name -> Varchar,
        move_resource_generic_type_params -> Jsonb,
        move_resource_data -> Jsonb,
    }
}

table! {
    script_function_payload (transaction_version) {
        transaction_version -> Int8,
        script_function_module_address -> Varchar,
        script_function_module_name -> Varchar,
        script_function_name -> Varchar,
        type_arguments -> Jsonb,
        arguments -> Jsonb,
    }
}

table! {
    script_payload (transaction_version) {
        transaction_version -> Int8,
        code -> Jsonb,
        type_arguments -> Jsonb,
        arguments -> Jsonb,
    }
}

table! {
    script_writeset_payload (transaction_version) {
        transaction_version -> Int8,
        execute_as -> Varchar,
        code -> Jsonb,
        type_arguments -> Jsonb,
        arguments -> Jsonb,
    }
}

table! {
    table_item_changes (transaction_version, transaction_index) {
        transaction_version -> Int8,
        transaction_index -> Int4,
        is_write -> Bool,
        state_key_hash -> Varchar,
        handle -> Varchar,
        key -> Varchar,
        value -> Varchar,
        table_data_key -> Jsonb,
        table_data_key_type -> Varchar,
        table_data_value -> Jsonb,
        table_data_value_type -> Varchar,
    }
}

table! {
    token_activities (event_key, sequence_number) {
        event_key -> Varchar,
        sequence_number -> Int8,
        account -> Varchar,
        token_id -> Nullable<Varchar>,
        event_type -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        created_at -> Timestamp,
        inserted_at -> Timestamp,
        transaction_hash -> Varchar,
    }
}

table! {
    token_datas (token_data_id) {
        token_data_id -> Varchar,
        creator -> Varchar,
        collection -> Varchar,
        name -> Varchar,
        description -> Varchar,
        max_amount -> Varchar,
        supply -> Int8,
        uri -> Varchar,
        royalty_payee_address -> Varchar,
        royalty_points_denominator -> Varchar,
        royalty_points_numerator -> Varchar,
        mutability_config -> Varchar,
        property_keys -> Varchar,
        property_values -> Varchar,
        property_types -> Varchar,
        minted_at -> Timestamp,
        last_minted_at -> Timestamp,
        inserted_at -> Timestamp,
    }
}

table! {
    token_propertys (token_id) {
        token_id -> Varchar,
        previous_token_id -> Varchar,
        property_keys -> Varchar,
        property_values -> Varchar,
        property_types -> Varchar,
        updated_at -> Timestamp,
        inserted_at -> Timestamp,
    }
}

table! {
    transactions (version) {
        #[sql_name = "type"]
        type_ -> Varchar,
        payload -> Jsonb,
        version -> Int8,
        hash -> Varchar,
        state_root_hash -> Varchar,
        event_root_hash -> Varchar,
        gas_used -> Int8,
        success -> Bool,
        vm_status -> Text,
        accumulator_root_hash -> Varchar,
        inserted_at -> Timestamp,
    }
}

table! {
    user_transactions (version) {
        version -> Int8,
        signature -> Jsonb,
        sender -> Varchar,
        sequence_number -> Int8,
        max_gas_amount -> Int8,
        expiration_timestamp_secs -> Timestamp,
        gas_unit_price -> Int8,
        timestamp -> Timestamp,
        inserted_at -> Timestamp,
    }
}

table! {
    write_set_changes (transaction_version, state_key_hash) {
        transaction_version -> Int8,
        state_key_hash -> Varchar,
        change_type -> Text,
        address -> Varchar,
        module -> Jsonb,
        resource -> Jsonb,
        data -> Jsonb,
        inserted_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    account_resources,
    block_metadata_transactions,
    blocks,
    collections,
    direct_writeset_payload,
    events,
    ledger_infos,
    metadatas,
    module_bundle_payload,
    module_changes,
    ownerships,
    processor_statuses,
    resource_changes,
    script_function_payload,
    script_payload,
    script_writeset_payload,
    table_item_changes,
    token_activities,
    token_datas,
    token_propertys,
    transactions,
    user_transactions,
    write_set_changes,
);
