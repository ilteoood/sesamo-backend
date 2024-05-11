use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    #[serde(rename = "messageId")]
    pub message_id: String,
}