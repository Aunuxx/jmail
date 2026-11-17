use serde::{Deserialize, Serialize};
use crate::types::internal::{Id, UTCDate};
use worker::wasm_bindgen::JsValue;


pub struct SQLFilter {
    pub clauses: Vec<String>,
    pub params: Vec<JsValue>,
}


pub type Header = (
    String,
    Option<String>,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailFilterCondition {
    pub in_mailbox: Option<Id>,
    pub in_mailbox_other_than: Option<Vec<Id>>,
    pub before: Option<UTCDate>,
    pub after: Option<UTCDate>,
    pub min_size: Option<u32>,
    pub max_size: Option<u32>,
    pub all_in_thread_have_keyword: Option<String>,
    pub some_in_thread_have_keyword: Option<String>,
    pub none_in_thread_have_keyword: Option<String>,
    pub has_keyword: Option<String>,
    pub not_keyword: Option<String>,
    pub has_attachment: Option<bool>, // TODO
    pub text: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub header: Option<Header>,
}



impl EmailFilterCondition {
    fn create_sql(self) -> SQLFilter {
        let mut clauses: Vec<String> = Vec::new();
        let mut params: Vec<JsValue> = Vec::new();

        if let Some(in_mailbox) = self.in_mailbox {
            clauses.push(format!("email_mailbox.id = ?{}", params.len() + 2));
            params.push(JsValue::from(in_mailbox.0));
        };
        if let Some(in_mailbox_other_than) = self.in_mailbox_other_than {
            let l = params.len() + 2;
            clauses.push(format!("email_mailbox.id NOT IN ({})", "?{}"));
            params.push(JsValue::from(in_mailbox_other_than)); // TODO
        }
        if let Some(before) = self.before {
            clauses.push(format!("emails.receivedAt < ?{}", params.len() + 2));
            params.push(JsValue::from(before));
        }
        if let Some(after) = self.after {
            clauses.push(format!("emails.receivedAt >= ?{}", params.len() + 2));
            params.push(JsValue::from(after));
        }
        if let Some(min_size) = self.min_size {
            clauses.push(format!("emails.size >= ?{}", params.len() + 2));
            params.push(JsValue::from(min_size));
        }
        if let Some(max_size) = self.max_size {
            clauses.push(format!("emails.size < ?{}", params.len() + 2));
            params.push(JsValue::from(max_size));
        }
        if let Some(keyword) = self.all_in_thread_have_keyword {
            clauses.push(format!("email.threadId IN (\
            SELECT threadId FROM email \
            GROUP BY threadId \
            HAVING COUNT(*) = SUM( \
                CASE WHEN keywords LIKE ?{} THEN 1 ELSE 0 END \
            )", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", keyword)));
        }
        if let Some(keyword) = self.some_in_thread_have_keyword {
            clauses.push(format!("email.threadId IN (\
            SELECT threadId FROM email \
            GROUP BY threadId \
            HAVING SUM( \
                CASE WHEN keywords LIKE ?{} THEN 1 ELSE 0 END \
            ) > 0", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", keyword)));
        }
        if let Some(keyword) = self.none_in_thread_have_keyword {
            clauses.push(format!("email.threadId IN (\
            SELECT threadId FROM email \
            GROUP BY threadId \
            HAVING SUM( \
                CASE WHEN keywords LIKE ?{} THEN 1 ELSE 0 END \
            ) = 0)", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", keyword)));
        }
        if let Some(keyword) = self.has_keyword {
            clauses.push(format!("emails.keywords LIKE ?{}", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", keyword)));
        }
        if let Some(keyword) = self.not_keyword {
            clauses.push(format!("emails.keywords NOT LIKE ?{}", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", keyword)));
        }
        if let Some(text) = self.text {
            clauses.push(format!(
                "(LIKE emails.from LIKE ?{0} \
                OR emails.to LIKE ?{0} \
                OR emails.cc LIKE ?{0} \
                OR emails.bcc LIKE ?{0} \
                OR emails.textBody LIKE ?{0} \
                OR emails.htmlBody LIKE ?{0})", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", text)));
        }
        if let Some(from) = self.from {
            clauses.push(format!("emails.from LIKE ?{}", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", from)));
        }
        if let Some(to) = self.to {
            clauses.push(format!("emails.to LIKE ?{}", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", to)));
        }
        if let Some(cc) = self.cc {
            clauses.push(format!("emails.cc LIKE ?{}", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", cc)));
        }
        if let Some(bcc) = self.bcc {
            clauses.push(format!("emails.bcc LIKE ?{}", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", bcc)));
        }
        if let Some(subject) = self.subject {
            clauses.push(format!("emails.subject LIKE ?{}", params.len() + 2));
            params.push(JsValue::from(format!("%{}%", subject)));
        }
        if let Some(body) = self.body {
            clauses.push(format!(
                "(emails.textBody LIKE ?{0} \
                OR emails.htmlBody LIKE ?{0})"
                , params.len() + 2));
            params.push(JsValue::from(format!("%{}%", body)));
        }
        if let Some(header) = self.header {
            match header.1 {
                None => {
                    clauses.push(format!("emails.partId = (\
                    SELECT partId FROM email_body_parts WHERE ?{} LIKE headers\
                    )", params.len() + 2));
                    params.push(JsValue::from(format!("%{}%", header.0)));
                }
                Some(val) => {
                    clauses.push(format!("emails.partId = (\
                    SELECT partId FROM email_body_parts WHERE ?{} LIKE headers\
                    )", params.len() + 2));
                    params.push(JsValue::from(format!("%{}_{}%", header.0, val))); // TODO
                }
            }
        }

        SQLFilter {
            clauses,
            params,
        }
    }
}
