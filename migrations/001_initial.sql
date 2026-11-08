CREATE TABLE IF NOT EXISTS accounts (
    id TEXT,
    hash TEXT, -- API hash
    email TEXT NOT NULL UNIQUE,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    can_read BOOLEAN NOT NULL DEFAULT FALSE,
    can_write BOOLEAN NOT NULL DEFAULT FALSE,
    can_delete BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id, hash)
);

CREATE TABLE IF NOT EXISTS mailboxes (
    id TEXT PRIMARY KEY ,
    account_id TEXT,
    name TEXT,
    can_read BOOLEAN NOT NULL DEFAULT FALSE,
    can_write BOOLEAN NOT NULL DEFAULT FALSE,
    can_delete BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);
