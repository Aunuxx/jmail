use worker::*;
use crate::jmap::types;
use crate::auth::api::API;
use hex;
use worker::wasm_bindgen::JsValue;
use crate::jmap::types::{D1Mailbox, EmailGetRequest, Id, Mailbox};

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

    pub async fn get_state(&self, api: &API, state_type: String) -> Result<types::State> {
        let statement = self.db.prepare(
            "SELECT session FROM state
            WHERE id = ?1"
        );
        let statement = statement.bind(&[
            hex::encode(api.hash).into()
        ])?;
        match statement.first::<u8>(Some("session")).await? {
            Some(state) => Ok(types::State { state: state.to_string() }),
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


}

