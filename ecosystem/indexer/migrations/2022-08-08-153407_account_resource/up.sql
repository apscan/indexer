-- Your SQL goes here
CREATE TABLE account_resources (
    address          VARCHAR(255) NOT NULL,
    hash             VARCHAR(255) NOT NULL,
    version          BIGINT       NOT NULL,
    type             TEXT         NOT NULL,
    
    PRIMARY KEY (address, hash)
);