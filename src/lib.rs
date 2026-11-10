mod jmap;
pub mod routes;
pub mod data;
pub use worker::*;
pub mod auth;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    ctx: Context,
) -> Result<Response> {
    routes::handle(req, env, ctx).await
}


