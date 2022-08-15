-- Your SQL goes here

/** Ex:
{
  "key": "0x0400000000000000000000000000000000000000000000000000000000000000000000000a550c18",
  "sequence_number": "0",
  "type": "0x1::reconfiguration::NewEpochEvent",
  "data": {
    "epoch": "1"
  }
}
 */

CREATE TABLE events
(
    -- join from "transactions"
    transaction_version BIGINT    NOT NULL,

    key              VARCHAR(100) NOT NULL,
    sequence_number  BIGINT       NOT NULL,
    type             TEXT         NOT NULL,
    data             jsonb        NOT NULL,

    -- Default time columns
    inserted_at      TIMESTAMP    NOT NULL DEFAULT NOW(),

    -- Constraints
    PRIMARY KEY (key, sequence_number),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE INDEX event_key_txn_version ON events (transaction_version);
CREATE INDEX event_key_seq_type_index ON events (key, sequence_number, type);

CREATE TABLE blocks
(
    transaction_version BIGINT NOT NULL,
    epoch               BIGINT NOT NULL,
    round               BIGINT NOT NULL,
    height              BIGINT NOT NULL UNIQUE PRIMARY KEY,
    hash                VARCHAR(255),
    time_microseconds   BIGINT NOT NULL,
    previous_block_votes jsonb NOT NULL,
    failed_proposer_indices jsonb NOT NULL
);

CREATE UNIQUE INDEX transaction_version_index ON events (transaction_version);