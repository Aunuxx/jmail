mod jmap;
pub mod routes;
pub mod data;
pub mod auth;

use serde_json::json;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    ctx: Context,
) -> Result<Response> {
    routes::handle(req, env, ctx).await
}


