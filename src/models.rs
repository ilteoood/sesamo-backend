use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenRequest {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "serverId")]
    pub server_id: String,
}


#[derive(Serialize, Deserialize)]
pub struct ServerDocument {
    pub name: String,
    pub r#type: String,
}