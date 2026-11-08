use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use worker::{console_log, Request};


#[derive(Serialize, Deserialize)]
pub struct API {
    pub hash: [u8; 32],
}

impl API {
    pub fn try_new(req: Request) -> Option<Self> {
        let auth_header = match req.headers().get("Authorization").unwrap() {
            Some(header) => header,
            None => return None,
        };
        let token = match auth_header.strip_prefix("Bearer ") {
            Some(t) => t.trim(),
            None => return None,
        };
        let digest = Sha256::digest(token.as_bytes()).into();
        Some(Self { hash: digest })
    }

}

