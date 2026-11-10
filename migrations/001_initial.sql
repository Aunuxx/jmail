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
    id TEXT PRIMARY KEY,
    accountId TEXT NOT NULL,
    name TEXT NOT NULL,
    parentId TEXT,
    roll TEXT,
    sortOrder INTEGER NOT NULL DEFAULT 0,
    totalEmails INTEGER NOT NULL DEFAULT 0,
    unreadEmails INTEGER NOT NULL DEFAULT 0,
    totalThreads INTEGER NOT NULL DEFAULT 0,
    unreadThreads INTEGER NOT NULL DEFAULT 0,
    myRights INTEGER NOT NULL DEFAULT 0, -- 8 squashed bools
    isSubscribed BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (accountId) REFERENCES accounts(id)
);
