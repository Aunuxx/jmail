mod types;
pub mod routes;
pub mod data;
pub use worker::*;
pub mod auth;

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    routes::handle(req, env).await
}


