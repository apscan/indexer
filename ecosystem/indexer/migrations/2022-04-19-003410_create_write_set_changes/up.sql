-- Your SQL goes here

CREATE TABLE write_set_changes
(
    -- join from "transactions"
    transaction_version          BIGINT       NOT NULL,
    state_key_hash             VARCHAR(255)   NOT NULL,

    change_type             TEXT         NOT NULL,
    address          VARCHAR(255) NOT NULL,

    module           jsonb        NOT NULL,
    resource         jsonb        NOT NULL,
    data             jsonb        NOT NULL,
    inserted_at      TIMESTAMP    NOT NULL DEFAULT NOW(),

    -- Constraints
    PRIMARY KEY (transaction_version, state_key_hash),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE INDEX write_set_changes_address_hash_version_index ON write_set_changes (address, state_key_hash, transaction_version);
