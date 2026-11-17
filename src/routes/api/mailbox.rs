use futures::future::ErrInto;
use crate::data::Data;
use crate::auth::api::API;
use crate::types::{requests, responses};
use responses::Responses;


pub async fn get(data: &Data, api: &API, args: serde_json::Value) -> Result<Responses, worker::Error> {
    let get_request: requests::GetRequest = serde_json::from_value(args)?;
    let (mailboxes, state) = futures::try_join!(
                        data.get_all_mailboxes(&get_request.account_id),
                        data.get_state(&api, "mailbox")
                        )?;
    Ok(Responses::MailboxGetResponse(responses::GetResponse {
        account_id: get_request.account_id,
        state,
        list: mailboxes,
        not_found: vec![],
    }))
}