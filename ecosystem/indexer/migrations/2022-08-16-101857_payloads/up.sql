-- Your SQL goes here
CREATE TABLE script_writeset_payload
(
    -- join from "transactions"
    transaction_version       BIGINT    NOT NULL,
    execute_as                VARCHAR(255) NOT NULL,
    code                      jsonb     NOT NULL,
    type_arguments            jsonb     NOT NULL,
    arguments                 jsonb     NOT NULL,


    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE TABLE direct_writeset_payload
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

CREATE TABLE script_function_payload
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

CREATE TABLE module_bundle_payload
(
    -- join from "transactions"
    transaction_version BIGINT       NOT NULL,
    module_changes      jsonb        NOT NULL,

    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);

CREATE TABLE script_payload
(
    -- join from "transactions"
    transaction_version BIGINT       NOT NULL,
    code                jsonb        NOT NULL,
    type_arguments      jsonb        NOT NULL,
    arguments           jsonb        NOT NULL,

    -- Constraints
    PRIMARY KEY (transaction_version),
    CONSTRAINT fk_transactions
        FOREIGN KEY (transaction_version)
            REFERENCES transactions (version)
);