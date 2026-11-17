use crate::data::Data;
use crate::auth::api::API;
use crate::types::{requests, responses, internal};
use responses::Responses;

type Error = String;

pub async fn get(data: &Data, api: &API, args: serde_json::Value) -> worker::Result<Responses, worker::Error> {
    // let get_request: EmailGetRequest = serde_json::from_value(args)?;
    // let emails = data.get_emails(&get_request).await?;
    // console_log!("emails: {:?}", emails);
    Ok(Responses::EmailGetResponse(responses::GetResponse {
        account_id: internal::Id("ab".to_owned()),
        state: data.get_state(&api, &*"email".to_string()).await?,
        list: Default::default(),
        not_found: vec![],
    }))
}

pub async fn query(data: &Data, api: &API, args: serde_json::Value) -> Result<Responses, worker::Error> {
    let request: requests::EmailQueryRequest = serde_json::from_value(args)?;
    let (emails, query_state) = futures::try_join!(
    data.query_emails(&request),
    data.get_state(&api, "email")
    )?;
    Ok(Responses::QueryResponse(responses::QueryResponse {
        account_id: request.query_request.account_id,
        query_state, // TODO
        can_calculate_changes: false,
        position: 0,
        ids: vec![],
        total: None,
        limit: None,
    }))
}