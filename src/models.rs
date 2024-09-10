use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub message_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenRequest {
    pub device_id: String,
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
        pub project_id: String,
    }
}
