use worker::{console_log, Context, Env, Request, Response};
use crate::data::Data;
use crate::auth::api::API;
use crate::jmap::types;
use crate::jmap::types::{MethodResponse, GetResponse, Id, GetRequest, EmailGetRequest};


pub async fn handle(
    mut _req: Request,
    _env: Env,
    _ctx: Context,
    _data: Data
) -> worker::Result<Response> {
    let api = match API::try_new(&_req) {
        Some(api) => api,
        None => return Response::error("API format invalid", 401),
    };
    let mut request: types::Request = _req.json().await?;
    console_log!("{:?}", request);
    let method_calls = std::mem::take(&mut request.method_calls);

    let mut response = types::Response {
        method_responses:  Vec::with_capacity(method_calls.len()),
        created_ids: None,
        session_state: _data.get_state(&api, "session".to_string()).await?,
    };

    for (method, args, id) in method_calls {
        let method_response: MethodResponse = (
            method.clone(),
            match method.as_str() {
                "Mailbox/get" => {
                    let get_request: GetRequest = serde_json::from_value(args)?;
                    let mailboxes = _data.get_all_mailboxes(&get_request.account_id).await?;
                    GetResponse {
                        account_id: get_request.account_id,
                        state: _data.get_state(&api, "mailbox".to_string()).await?,
                        list: serde_json::to_value(mailboxes)?,
                        not_found: vec![],
                    }
                }
                "Email/get" => {
                    let get_request: EmailGetRequest = serde_json::from_value(args)?;
                    let emails = _data.get_emails(&get_request).await?;
                    GetResponse {
                        account_id: Id("ab".to_owned()),
                        state: _data.get_state(&api, "email".to_string()).await?,
                        list: Default::default(),
                        not_found: vec![],
                    }
                }
                _ => {
                    console_log!("{:?}", method);
                    return Response::error("Method not found", 404); // TODO
                }
            },
            id,
        );
        response.method_responses.push(method_response);
    }
    console_log!("{:?}", response);
    Response::from_json(&response)
}

