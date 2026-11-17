use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::internal::{Id, State};
use crate::types::objects::{Email, Mailbox};



#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetResponse<T> {
    pub account_id: Id,
    pub state: State,
    pub list: Vec<T>,
    pub not_found: Vec<Id>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResponse {
    pub account_id: Id,
    pub query_state: State,
    #[serde(deserialize_with = "deserialize_bool")]
    pub can_calculate_changes: bool,
    pub position: u32,
    pub ids: Vec<Id>,
    pub total: Option<u32>,
    pub limit: Option<u32>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Responses {
    MailboxGetResponse(GetResponse<Mailbox>),
    EmailGetResponse(GetResponse<Email>),
    QueryResponse(QueryResponse),
}

pub type MethodResponse = (
    String,
    Responses,
    Id,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub method_responses: Vec<MethodResponse>,
    pub created_ids: Option<HashMap<Id, Id>>,
    pub session_state: State,
}