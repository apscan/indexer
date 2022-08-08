-- Your SQL goes here
drop table if exists write_set_changes; 
CREATE TABLE write_set_changes
(
    -- join from "transactions"
    transaction_hash VARCHAR(255) NOT NULL,
    version          BIGINT       NOT NULL,
    hash             VARCHAR(255) NOT NULL,

    type             TEXT         NOT NULL,
    address          VARCHAR(255) NOT NULL,

    module           jsonb        NOT NULL,
    resource         jsonb        NOT NULL,
    data             jsonb        NOT NULL,
    inserted_at      TIMESTAMP    NOT NULL DEFAULT NOW(),

    -- Constraints
    PRIMARY KEY (transaction_hash, hash),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_hash)
            REFERENCES transactions (hash)
);

CREATE INDEX write_set_changes_tx_hash_addr_type_index ON write_set_changes (transaction_hash, address, type);
CREATE INDEX write_set_changes_address_version_type_index ON write_set_changes (address, version, type);
