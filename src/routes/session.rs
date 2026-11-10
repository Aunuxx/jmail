use worker::{Context, Env, Request, Response};
use serde_json::{json, Map};
use crate::data::Data;
use crate::auth::api::API;
use crate::console_log;

pub async fn handle(
    _req: Request,
    _env: Env,
    _ctx: Context,
    _data: Data
) -> worker::Result<Response> {
    let api = match API::try_new(&_req) {
        Some(api) => api,
        None => return Response::error("API format invalid", 401),
    };
    let accounts = _data.get_accounts(&api).await?;
    if accounts.is_empty() {
        return Response::error("Invalid API key", 401);
    }
    let primary =  match accounts.iter().find(|account|account.is_primary).cloned() {
        Some(account) => account,
        None => return Response::error("No Primary Account", 403),
    };

    let state = _data.get_state(&api, "session".to_string()).await?;

    let mut account_map = Map::new();

    for account in accounts {
        account_map.insert(
            account.id,
            json!({
                "name": account.email,
                "isReadOnly": !account.can_read,
                "isPersonal": true,
                "accountCapabilities": {
                    "urn:ietf:params:jmap:mail": {}
                }
            }),
        );
    }
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
            "accounts": account_map,
            "primaryAccounts": { "urn:ietf:params:jmap:mail": primary.id },
            "username": primary.email,
            "apiUrl": "http://192.168.1.33:8787/api",
            "downloadUrl": "http://192.168.1.33:8787/download/{accountId}/{blobId}/{name}?accept={type}",
            "uploadUrl": "http://192.168.1.33:8787/upload/{accountId}/",
            "eventSourceUrl": "http://192.168.1.33:8787/eventsource/?types={types}&closeafter={closeafter}&ping={ping}",
            "state": state.state,
        });

    Response::from_json(&response)
}