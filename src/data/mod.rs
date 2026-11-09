mod d1;
mod r2;

// pub mod repositories;
// pub mod traits;
use worker::{Bucket, Env, Result};
use crate::auth::api::API;
use crate::data::d1::D1Store;
use crate::jmap::types;

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

    pub async fn get_state(&self, api: &API) -> Result<types::State> {
        self.db.get_state(api).await
    }
}
