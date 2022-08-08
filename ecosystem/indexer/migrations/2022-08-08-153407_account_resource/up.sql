-- Your SQL goes here
CREATE TABLE latset_write_set_changes (
    version          BIGINT       NOT NULL,
    hash             VARCHAR(255) NOT NULL,
    address          VARCHAR(255) NOT NULL,
    type             TEXT         NOT NULL,
    
    PRIMARY KEY (address, hash)
);
