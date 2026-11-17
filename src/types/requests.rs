use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::internal::Id;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub using: Vec<String>,
    pub method_calls: Vec<(String, serde_json::Value, Id)>, // TODO
    pub created_ids: Option<HashMap<Id, Id>>,
}


// GET
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRequest {
    pub account_id: Id,
    pub ids: Option<Vec<Id>>,
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailGetRequest {
    #[serde(flatten)]
    pub get_request: GetRequest,
    // [ "id", "blobId", "threadId", "mailboxIds", "keywords", "size",
    //  "receivedAt", "messageId", "inReplyTo", "references", "sender", "from",
    //  "to", "cc", "bcc", "replyTo", "subject", "sentAt", "hasAttachment",
    //  "preview", "bodyValues", "textBody", "htmlBody", "attachments" ]
    pub body_properties: Option<Vec<String>>,
    // [ "partId", "blobId", "size", "name", "type", "charset",
    //            "disposition", "cid", "language", "location" ]
    pub fetch_text_body_values: Option<bool>, // false
    #[serde(rename = "fetchHTMLBodyValues")]
    pub fetch_html_body_values: Option<bool>, // false
    pub fetch_all_body_values: Option<bool>, // false
    pub max_body_value_bytes: Option<u32>, // 0

}


// QUERY
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRequest {
    pub account_id: Id,
    pub filter: FilterOpCo,
    pub sort: Option<Vec<Comparator>>,
    pub position: i32,
    pub anchor: Option<Id>,
    pub anchor_offset: Option<i32>,
    pub limit: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub calculate_total: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    AND,
    OR,
    NOT
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterOpCo {
    FilterOperator(FilterOperator),
    EmailFilterCondition(EmailFilterCondition),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterOperator {
    pub operator: Operator,
    pub conditions: Vec<FilterOpCo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComparatorProperty {
    ReceivedAt,
    Size,
    From,
    To,
    Subject,
    SentAt,
    HasKeyword,
    AllInThreadHaveKeyword,
    SomeInThreadHaveKeyword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comparator {
    pub property: ComparatorProperty,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_ascending: bool,
    pub collation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailQueryRequest {
    #[serde(flatten)]
    pub query_request: QueryRequest,
    #[serde(deserialize_with = "deserialize_bool")]
    pub collapse_threads: bool,
}
