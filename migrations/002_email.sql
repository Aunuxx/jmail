CREATE TABLE IF NOT EXISTS emails (
    -- JMAP
    id TEXT PRIMARY KEY,
    blobId TEXT NOT NULL,
    threadId TEXT NOT NULL,
    mailboxIds TEXT NOT NULL,
    keywords TEXT NOT NULL,
    size INTEGER NOT NULL,
    receivedAt TEXT NOT NULL,
    -- MESSAGE
    messageId TEXT,
    inReplyTo TEXT,
    "references" TEXT,
    sender TEXT,
    "from" TEXT,
    "to" TEXT,
    cc TEXT,
    bcc TEXT,
    replyTo TEXT,
    subject TEXT,
    sentAt TEXT,
    -- STRUCTURE
    bodyStructure TEXT NOT NULL, -- ID
    bodyValues TEXT NOT NULL,
    textBody TEXT NOT NULL,
    htmlBody TEXT NOT NULL,
    attachments TEXT NOT NULL,
    hasAttachment BOOLEAN NOT NULL,
    preview TEXT,
    FOREIGN KEY (bodyStructure) REFERENCES email_body_parts(partId)
);


CREATE TABLE IF NOT EXISTS email_body_parts (
    partId TEXT PRIMARY KEY,
    blobId TEXT NOT NULL,
    size Integer NOT NULL,
    headers TEXT NOT NULL,
    name TEXT,
    type TEXT NOT NULL,
    charset TEXT,
    disposition TEXT,
    cid TEXT,
    language TEXT,
    location TEXT
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
