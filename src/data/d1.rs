use worker::*;
use crate::jmap::types;
use crate::auth::api::API;
use hex;

pub struct D1Store {
    db: D1Database,
}

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

    pub async fn get_state(&self, api: &API) -> Result<types::State> {
        let statement = self.db.prepare(
            "SELECT state FROM state
            WHERE id = ?1"
        );
        let statement = statement.bind(&[
            hex::encode(api.hash).into()
        ])?;
        match statement.first::<types::State>(None).await? {
            Some(state) => Ok(state),
            None => Err("State not found".into()),
        }
    }

    // pub async fn init(&self) -> Result<()> {
    //     let file = include_str!("migrations/001_initial.sql");
    //     self.db.exec(file).await?;
    //     let file = include_str!("migrations/002_email.sql");
    //     self.db.exec(file).await?;
    //     Ok(())
    // }
}

