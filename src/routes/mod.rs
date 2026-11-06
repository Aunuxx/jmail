use worker::*;

mod session;
// pub mod api;
// pub mod upload;
// pub mod download;

pub async fn handle(
    req: Request,
    env: Env,
    ctx: Context,
) -> Result<Response> {
    let path = req.path();

    match path.as_str() {
        "/.well-known/jmap" => {
            session::handle(req, env, ctx).await
        }
        "/" => {
            Response::ok("")
        }
        // "/api/" => {
        //     api::handle(req, env, ctx).await
        // }
        // _ if path.starts_with("/upload/") => {
        //     upload::handle(req, env, ctx).await
        // }
        // _ if path.starts_with("/download/") => {
        //     download::handle(req, env, ctx).await
        // }
        _ => Response::error("Not found", 404),
    }
}
