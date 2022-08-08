table! {
    block_metadata_transactions (hash) {
        hash -> Varchar,
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
    collections (collection_id) {
        collection_id -> Varchar,
        creator -> Varchar,
        name -> Varchar,
        description -> Varchar,
        max_amount -> Nullable<Int8>,
        uri -> Varchar,
        created_at -> Timestamp,
        inserted_at -> Timestamp,
    }
}

table! {
    events (key, sequence_number) {
        transaction_hash -> Varchar,
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
        max_amount -> Int8,
        supply -> Int8,
        uri -> Varchar,
        royalty_payee_address -> Varchar,
        royalty_points_denominator -> Int8,
        royalty_points_numerator -> Int8,
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
    transactions (hash) {
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
    user_transactions (hash) {
        hash -> Varchar,
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
    write_set_changes (transaction_hash, hash) {
        transaction_hash -> Varchar,
        version -> Int8,
        hash -> Varchar,
        #[sql_name = "type"]
        type_ -> Text,
        address -> Varchar,
        module -> Jsonb,
        resource -> Jsonb,
        data -> Jsonb,
        inserted_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    block_metadata_transactions,
    collections,
    events,
    ledger_infos,
    metadatas,
    ownerships,
    processor_statuses,
    token_activities,
    token_datas,
    token_propertys,
    transactions,
    user_transactions,
    write_set_changes,
);
