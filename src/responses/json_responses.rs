use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonResponse {
    status: String,
    message: String,
}

impl JsonResponse {
    pub fn new(status: &str, message: &str) -> Self {
        Self {
            status: status.to_string(),
            message: message.to_string(),
        }
    }

    pub fn to_json(&self) {
        serde_json::to_string(self).unwrap();
    }

    pub fn to_json_pretty(&self) {
        serde_json::to_string_pretty(self).unwrap();
    }
}
