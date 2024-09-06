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

pub mod firebase {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct ObjectConfiguration {
        #[serde(alias = "_firestore_id")]
        pub id: String,
        pub body: String,
        pub url: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ServerAllowedDevices {
        pub list: Vec<String>,
    }

    pub struct ServerDocumentConfiguration {
        pub allowed_devices: ServerAllowedDevices,
        pub objects: HashMap<String, ObjectConfiguration>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ServerDocumentBase {
        #[serde(alias = "_firestore_id")]
        pub id: String,
        pub name: String,
        pub r#type: String,
    }

    #[derive(Serialize, Deserialize, Clone, Copy)]
    pub enum ServerDocumentType {
        HttpPost,
    }

    pub struct ServerDocument {
        pub id: String,
        pub r#type: ServerDocumentType,
        pub configurations: ServerDocumentConfiguration,
    }

    #[derive(Deserialize, Serialize)]
    pub struct FirestoreServiceAccount {
        #[serde(rename = "type")]
        pub service_account_type: String,
        #[serde(rename = "project_id")]
        pub project_id: String,
        #[serde(rename = "private_key_id")]
        pub private_key_id: String,
        #[serde(rename = "private_key")]
        pub private_key: String,
        #[serde(rename = "client_email")]
        pub client_email: String,
        #[serde(rename = "client_id")]
        pub client_id: String,
        #[serde(rename = "auth_uri")]
        pub auth_uri: String,
        #[serde(rename = "token_uri")]
        pub token_uri: String,
        #[serde(rename = "auth_provider_x509_cert_url")]
        pub auth_provider_x509_cert_url: String,
        #[serde(rename = "client_x509_cert_url")]
        pub client_x509_cert_url: String,
        #[serde(rename = "universe_domain")]
        pub universe_domain: String,
    }
}
