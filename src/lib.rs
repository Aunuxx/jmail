mod jmap;
use worker::*;

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let id = jmap::types::Id::try_from("test".to_string());
    console_log!("{}", id?.as_str());
    Response::ok("Hello World! yay")
}


