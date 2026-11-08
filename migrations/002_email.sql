CREATE TABLE IF NOT EXISTS emails (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    blob_id TEXT NOT NULL,
    thread_id TEXT,
    subject TEXT,
    sender TEXT,
    received_at TEXT NOT NULL,
    size INT NOT NULL,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);

CREATE TABLE IF NOT EXISTS email_mailbox (
    email_id TEXT NOT NULL,
    mailbox_id TEXT NOT NULL,
    isSeen BOOLEAN NOT NULL DEFAULT FALSE,
    isFlagged BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (email_id, mailbox_id),
    FOREIGN KEY (email_id) REFERENCES emails(id),
    FOREIGN KEY (mailbox_id) REFERENCES accounts(id)
);
