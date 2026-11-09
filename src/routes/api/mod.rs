use worker::{console_log, Context, Env, Request, Response};
use serde_json::{json, Map};
use crate::data::Data;
use crate::auth::api::API;
use crate::jmap::types;

pub async fn handle(
    mut _req: Request,
    _env: Env,
    _ctx: Context,
    _data: Data
) -> worker::Result<Response> {
    let requests: types::Request = _req.json().await?;
    for (method, args, id) in &requests.method_calls {
        
    }

    console_log!("{:?}", requests.method_calls);
    Response::ok("")
}

