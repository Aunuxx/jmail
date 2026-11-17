use worker::{console_log, Context, Env, Request, Response};
use crate::data::Data;
use crate::auth::api::API;
use crate::jmap::types;
use crate::jmap::types::{MethodResponse, GetResponse, Id, GetRequest, Responses, EmailQueryRequest};

pub async fn handle(
    mut _req: Request,
    _env: Env,
    _ctx: Context,
    data: Data
) -> worker::Result<Response> {
    let api = match API::try_new(&_req) {
        Some(api) => api,
        None => return Response::error("API format invalid", 401),
    };
    let mut request: types::Request = _req.json().await?;
    let method_calls = std::mem::take(&mut request.method_calls);

    let mut response = types::Response {
        method_responses: Vec::with_capacity(method_calls.len()),
        created_ids: None,
        session_state: data.get_state(&api, "session").await?,
    };

    for (method, args, id) in method_calls {
        // console_log!("{:?} {:?}", method, args);
        let method_response: MethodResponse = (
            method.clone(),
            match method.as_str() {
                "Mailbox/get" => {
                    let get_request: GetRequest = serde_json::from_value(args)?;
                    let (mailboxes, state) = futures::try_join!(
                        data.get_all_mailboxes(&get_request.account_id),
                        data.get_state(&api, "mailbox")
                        )?;
                    Responses::MailboxGetResponse(GetResponse {
                        account_id: get_request.account_id,
                        state,
                        list: mailboxes,
                        not_found: vec![],
                    })
                }
                "Email/get" => {
                    return Response::error(format!("{:?}", args), 400);
                    // let get_request: EmailGetRequest = serde_json::from_value(args)?;
                    // let emails = _data.get_emails(&get_request).await?;
                    // console_log!("emails: {:?}", emails);
                    // GetResponse {
                    //     account_id: Id("ab".to_owned()),
                    //     state: _data.get_state(&api, "email".to_string()).await?,
                    //     list: Default::default(),
                    //     not_found: vec![],
                    // }
                }
                "Email/query" => {
                    let request: EmailQueryRequest = serde_json::from_value(args)?;
                    let (emails, query_state) = futures::try_join!(
                        data.query_emails(&request),
                        data.get_state(&api, "email")
                    )?;
                    Responses::QueryResponse(types::QueryResponse {
                        account_id: request.query_request.account_id,
                        query_state, // TODO
                        can_calculate_changes: false,
                        position: 0,
                        ids: vec![],
                        total: None,
                        limit: None,
                    })
                }
                _ => {
                    console_log!("{:?} not found", method);
                    return Response::error(format!("Method not found: {}", method), 404); // TODO
                }
            },
            id,
        );
        response.method_responses.push(method_response);
    }
    Response::from_json(&response)
}

