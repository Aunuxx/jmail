use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::internal::{Id, UTCDate};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_primary: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub can_read: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    can_write: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    can_delete: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxRights {
    pub may_read_items: bool,
    pub may_remove_items: bool,
    pub may_set_seen: bool,
    pub may_set_keywords: bool,
    pub may_create_child: bool,
    pub may_rename: bool,
    pub may_delete: bool,
    pub may_submit: bool,
}

impl From<u8> for MailboxRights {
    fn from(value: u8) -> Self {
        MailboxRights {
            may_read_items: (value & 0x01) != 0,
            may_remove_items: (value & 0x02) != 0,
            may_set_seen: (value & 0x04) != 0,
            may_set_keywords: (value & 0x08) != 0,
            may_create_child: (value & 0x10) != 0,
            may_rename: (value & 0x20) != 0,
            may_delete: (value & 0x40) != 0,
            may_submit: (value & 0x80) != 0,
        }
    }
}

impl From<MailboxRights> for u8 {
    fn from(m: MailboxRights) -> Self {
        let mut value: u8 = 0;
        value |= u8::from(m.may_read_items);
        value |= u8::from(m.may_remove_items) << 1;
        value |= u8::from(m.may_set_seen) << 2;
        value |= u8::from(m.may_set_keywords) << 3;
        value |= u8::from(m.may_create_child) << 4;
        value |= u8::from(m.may_rename) << 5;
        value |= u8::from(m.may_delete) << 6;
        value |= u8::from(m.may_submit) << 7;
        value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mailbox {
    pub id: Id,
    pub name: String,
    pub parent_id: Option<Id>,
    pub roll: Option<String>,
    pub sort_order: u32,
    pub total_emails: u32,
    pub unread_emails: u32,
    pub total_threads: u32,
    pub unread_threads: u32,
    pub my_rights: MailboxRights,
    pub is_subscribed: bool,
}

impl From<D1Mailbox> for Mailbox {
    fn from(m: D1Mailbox) -> Self {
        Self {
            id: m.id,
            name: m.name,
            parent_id: m.parent_id,
            roll: m.roll,
            sort_order: m.sort_order,
            total_emails: m.total_emails,
            unread_emails: m.unread_emails,
            total_threads: m.total_threads,
            unread_threads: m.unread_threads,
            my_rights: MailboxRights::from(m.my_rights),
            is_subscribed: m.is_subscribed,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct D1Mailbox {
    pub id: Id,
    pub account_id: Id,
    pub name: String,
    pub parent_id: Option<Id>,
    pub roll: Option<String>,
    pub sort_order: u32,
    pub total_emails: u32,
    pub unread_emails: u32,
    pub total_threads: u32,
    pub unread_threads: u32,
    pub my_rights: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub is_subscribed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailBodyPart {
    pub part_id: Option<String>,
    pub blob_id: Option<Id>,
    pub size: u32,
    pub headers: Vec<EmailHeader>,
    pub name: Option<String>,
    #[serde(rename = "Type")]
    pub part_type: String,
    pub charset: Option<String>,
    pub disposition: Option<String>,
    pub cid: Option<String>,
    pub language: Option<Vec<String>>,
    pub location: Option<String>,
    pub sub_parts: Option<Vec<EmailBodyPart>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailBodyValue {
    pub value: String,
    pub is_encoding_problem: bool, // false
    pub is_truncated: bool, // false, true if value has been truncated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    // JMAP
    pub id: Id,
    pub blob_id: Id,
    pub thread_id: Id,
    pub mailbox_ids: HashMap<Id, bool>, // TODO: bool deserial issues?
    pub keywords: HashMap<String, bool>,
    pub size: u32,
    pub received_at: UTCDate,
    // MESSAGE
    pub message_id: Option<Vec<String>>,
    pub in_reply_to: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
    pub sender: Option<Vec<String>>,
    pub from: Option<Vec<String>>,
    pub to: Option<Vec<String>>,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub reply_to: Option<Vec<String>>,
    pub subject: Option<String>,
    pub sent_at: Option<UTCDate>,
    // STRUCTURE
    pub body_structure: EmailBodyPart,
    pub body_values: HashMap<String, EmailBodyValue>,
    pub text_body: Vec<EmailBodyPart>,
    pub html_body: Vec<EmailBodyPart>,
    pub attachments: Vec<EmailBodyPart>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub has_attachment: bool,
    pub preview: String,
}
