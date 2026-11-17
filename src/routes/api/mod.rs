mod mailbox;
mod email;

use worker::{console_log, Context, Env, Request, Response};
use crate::data::Data;
use crate::auth::api::API;
use crate::types::{internal, requests, responses};
use responses::Responses;
use crate::types::internal::MethodObjects;

pub async fn handle(
    mut req: Request,
    data: Data
) -> worker::Result<Response> {
    let api = match API::try_new(&req) {
        Some(api) => api,
        None => return Response::error("API format invalid", 401),
    };
    let mut request: requests::Request = req.json().await?;
    let method_calls = std::mem::take(&mut request.method_calls);

    let mut response = responses::Response {
        method_responses: Vec::with_capacity(method_calls.len()),
        created_ids: None,
        session_state: data.get_state(&api, "session").await?,
    };

    for (method, args, id) in method_calls {
        let (method_object, method_type) : (MethodObjects, &str) = method.split_once('/').unwrap_or((None, ""));
        let method_response: responses::MethodResponse = (
            method,
            match method_object {
                MethodObjects::Mailbox => {
                    match method_type {
                        "get" => {mailbox::get(&data, &api, args).await?},
                        _ => {return Response::error(format!("Method not found: {:?}", method_object), 404);}
                    }
                }
                MethodObjects::Email => {
                    match method_type {
                        "get" => {email::get(&data, &api, args).await?},
                        "query" => {email::query(&data, &api, args).await?},
                        _ => {return Response::error(format!("Method not found: {:?}", method_object), 404);}
                    }
                }
                _ => {
                    console_log!("{:?} not found", method_object);
                    return Response::error(format!("datatype not found: {}", method_type), 404); // TODO
                }
            },
            id,
        );
        response.method_responses.push(method_response);
    }
    Response::from_json(&response)
}

