mod d1;
mod r2;

// pub mod repositories;
// pub mod traits;
use worker::{console_log, Bucket, Env, Result};
use crate::auth::api::API;
use crate::data::d1::D1Store;
use crate::jmap::types;
use crate::jmap::types::{Email, EmailBodyPart, EmailGetRequest, Id, Mailbox};

pub struct Data {
    db: D1Store,
    bucket: Bucket,
}

impl Data {
    pub fn new(env: &Env) -> Result<Self> {
        Ok(Self {
            db: D1Store::new(env.d1("jmail_d1")?),
            bucket: env.bucket("jmail_bucket")?,
        })
    }
    
    pub async fn get_accounts(&self, api: &API) -> Result<Vec<types::Account>> {
        self.db.get_accounts(api).await
    }

    pub async fn get_state(&self, api: &API, state_type: String) -> Result<types::State> {
        self.db.get_state(api, state_type).await
    }

    pub async fn get_all_mailboxes(&self, account_id: &Id) -> Result<Vec<Mailbox>> {
        self.db.get_all_mailboxes(account_id).await
    }

    pub async fn get_emails(&self, request: &EmailGetRequest) -> Result<Vec<Email>> {
        let properties: Vec<String> = request.properties.unwrap_or_else(|| ["id", "blobId", "threadId", "mailboxIds", "keywords", "size", "receivedAt", "messageId", "inReplyTo", "references", "sender", "from", "to", "cc", "bcc", "replyTo", "subject", "sentAt", "hasAttachment", "preview", "bodyValues", "textBody", "htmlBody", "attachments"].into());
        let body_properties: Vec<String> = request.body_properties.unwrap_or_else(|| ["partId", "blobId", "size", "name", "type", "charset", "disposition", "cid", "language", "location"].into());
        let fetch_text_body_values = request.fetch_text_body_values.unwrap_or(false);
        let fetch_html_body_values = request.fetch_html_body_values.unwrap_or(false);
        let fetch_all_body_values = request.fetch_all_body_values.unwrap_or(false);
        let max_body_value_bytes = request.max_body_value_bytes.unwrap_or(0);
        
    }
}
