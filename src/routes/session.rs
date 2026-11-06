use worker::{console_log, Context, Env, Request, Response};
use serde_json::json;

pub async fn handle(
    req: Request,
    env: Env,
    ctx: Context,
) -> worker::Result<Response> {
    let auth_header = match req.headers().get("Authorization")? {
        Some(header) => header,
        None => return Response::error("Missing Authorization Header", 401),
    };
    let token = match auth_header.strip_prefix("Bearer ") {
        Some(t) => t.trim(),
        None => return Response::error("Invalid Authorization Format. Expected Bearer token.", 401),
    };
    console_log!("{}", token);
    let response =
        json!({
            "capabilities": {
                "urn:ietf:params:jmap:core": {
                    "maxSizeUpload": 50000000,
                    "maxConcurrentUpload": 4,
                    "maxSizeRequest": 10000000,
                    "maxConcurrentRequests": 4,
                    "maxCallsInRequest": 16,
                    "maxObjectsInGet": 500,
                    "maxObjectsInSet": 500,
                    "collationAlgorithms": []
                },
            },
            "accounts": {},
            "primaryAccounts": {},
            "username": "user@example.com",
            "apiUrl": "http://192.168.1.33:8787/api/",
            "downloadUrl": "http://192.168.1.33:8787/download/{accountId}/{blobId}/{name}?accept={type}",
            "uploadUrl": "http://192.168.1.33:8787/upload/{accountId}/",
            "eventSourceUrl": "http://192.168.1.33:8787/eventsource/?types={types}&closeafter={closeafter}&ping={ping}",
            "state": "0"
        });
return Response::from_json(&response);
}