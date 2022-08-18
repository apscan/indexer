-- Your SQL goes here
CREATE TABLE script_write_set_payloads
(
    -- join from "transactions"
    transaction_version       BIGINT    NOT NULL,
    execute_as                VARCHAR(255) NOT NULL,
    code                      VARCHAR     NOT NULL,
    abi                       jsonb     NOT NULL,
    type_arguments            jsonb     NOT NULL,
    arguments                 jsonb     NOT NULL,


    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE TABLE direct_write_set_payloads
(
    -- join from "transactions"
    transaction_version       BIGINT    NOT NULL,
    events                    jsonb     NOT NULL,
    changes                   jsonb     NOT NULL,                       


    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE TABLE script_function_payloads
(
    -- join from "transactions"
    transaction_version                 BIGINT       NOT NULL,
    script_function_module_address      VARCHAR(255) NOT NULL,
    script_function_module_name         VARCHAR(255) NOT NULL,
    script_function_name                VARCHAR      NOT NULL,
    type_arguments                      jsonb        NOT NULL,
    arguments                           jsonb        NOT NULL,                           


    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE TABLE module_bundle_payloads
(
    -- join from "transactions"
    transaction_version BIGINT       NOT NULL,
    modules              jsonb        NOT NULL,

    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE TABLE script_payloads
(
    -- join from "transactions"
    transaction_version BIGINT       NOT NULL,
    code                VARCHAR      NOT NULL,
    abi                 jsonb        NOT NULL,
    type_arguments      jsonb        NOT NULL,
    arguments           jsonb        NOT NULL,

    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);
