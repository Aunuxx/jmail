use worker::*;
use crate::jmap::types;
use crate::auth::api::API;
use hex;
use worker::wasm_bindgen::JsValue;
use crate::jmap::types::{D1Mailbox, Email, EmailGetRequest, Id, Mailbox};

pub struct D1Store {
    db: D1Database,
}

// TODO: ADD PERMISSIONS

impl D1Store {
    pub fn new(db: D1Database) -> D1Store {
        Self { db }
    }

    pub async fn get_accounts(&self, api: &API) -> Result<Vec<types::Account>> {
        let statement = self.db.prepare(
            "SELECT * FROM accounts
             WHERE hash = ?1"
        );
        let statement = statement.bind(&[
            hex::encode(api.hash).into()
        ])?;
        let result = statement.all().await?;
        Ok(result.results::<types::Account>()?)
    }

    pub async fn get_state(&self, api: &API, state_type: &str) -> Result<types::State> {
        let statement = self.db.prepare(
            format!("SELECT {} FROM state
            WHERE id = ?1", state_type)
        );
        let statement = statement.bind(&[
            hex::encode(api.hash).into()
        ])?;
        match statement.first::<u8>(Some(state_type)).await? { // TODO
            Some(state) => Ok(types::State(state.to_string())),
            None => Err("State not found".into()),
        }
    }

    pub async fn get_all_mailboxes(&self, account_id: &Id) -> Result<Vec<Mailbox>> {
        let statement = self.db.prepare(
            "SELECT * FROM mailboxes
            WHERE accountId = ?1"
        );
        let statement = statement.bind(&[
            JsValue::from(account_id.0.clone()),
        ])?;
        let result = statement.all().await?;
        let rows = result.results::<D1Mailbox>()?;
        Ok(rows.into_iter().map(Mailbox::from).collect())
    }

    pub  async fn get_emails(&self, request: &EmailGetRequest) -> Result<Vec<Email>> {
        let properties: Vec<String> = request.get_request.properties
            .clone()
            .unwrap_or_else(||
                ["id", "blobId", "threadId", "mailboxIds", "keywords", "size", "receivedAt",
                    "messageId", "inReplyTo", "references", "sender", "from", "to", "cc", "bcc",
                    "replyTo", "subject", "sentAt", "hasAttachment", "preview", "bodyValues",
                    "textBody", "htmlBody", "attachments"]
                    .map(String::from)
                    .to_vec());
        // let body_properties: Vec<String> = request.body_properties.unwrap_or_else(|| ["partId", "blobId", "size", "name", "type", "charset", "disposition", "cid", "language", "location"].into());
        // let fetch_text_body_values = request.fetch_text_body_values.unwrap_or(false);
        // let fetch_html_body_values = request.fetch_html_body_values.unwrap_or(false);
        // let fetch_all_body_values = request.fetch_all_body_values.unwrap_or(false);
        // let max_body_value_bytes = request.max_body_value_bytes.unwrap_or(0);

        let statement = self.db.prepare(
            "SELECT ?1 FROM emails"
        );
        let statement = statement.bind(&[
            JsValue::from(properties)
        ])?;
        let result = statement.all().await?;
        Ok(result.results::<Email>()?)
    }
    pub async fn query_emails(&self, request: &types::EmailQueryRequest) -> Result<Vec<Id>> {

        let statement = self.db.prepare(
            "SELECT DISTINCT id FROM emails
            JOIN email_mailbox ON emails.id = email_mailbox.email_id
            JOIN mailboxes ON email_mailbox.mailbox_id = mailboxes.id
            WHERE mailboxes.accountId = ?1
            AND (
                -- filters will be added here
            )"
        );
        let statement = statement.bind(&[
            // JsValue::from(request.query_request.account_id.0)
        ])?;
        Ok(vec![Id("a".parse()?)])
    }
}

