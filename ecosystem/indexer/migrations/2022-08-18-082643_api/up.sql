-- Your SQL goes here
create schema api;
create role apscan_api noinherit login password 'aptos1234';
grant usage on schema api to apscan_api;

CREATE OR REPLACE view api.blocks as 
    SELECT blk.transaction_version, blk.epoch, blk.round, 
    blk.height, blk.hash, blk.time_microseconds, blk.previous_block_votes,
    blk.failed_proposer_indices, blk_txn.proposer, 
    prev_blk.transaction_version as prev_block_version, 
    prev_blk.hash as prev_block_hash,
    next_blk.transaction_version as next_block_version,
    next_blk.hash as next_block_hash,
    (SELECT sum(gas_fee) FROM (SELECT u_tx.gas_unit_price * tx.gas_used as gas_fee
    FROM public.user_transactions as u_tx
    INNER JOIN public.transactions as tx
    on u_tx.version = tx.version
    WHERE u_tx.version > prev_blk.transaction_version 
        and COALESCE(u_tx.version < next_blk.transaction_version, true)
    ) as  gas_fee ) as fees
    FROM public.blocks as blk
    LEFT JOIN public.block_metadata_transactions as blk_txn
    ON blk.transaction_version = blk_txn.version
    LEFT JOIN public.blocks as prev_blk
    ON blk.height -1 = prev_blk.height
    LEFT JOIN public.blocks as next_blk
    ON blk.height + 1 = next_blk.height    
    ORDER BY transaction_version desc;

grant select on api.blocks to apscan_api;
