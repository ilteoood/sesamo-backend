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

    impl ServerAllowedDevices {
        pub fn default() -> Self {
            Self { list: vec![] }
        }
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
        #[serde(rename = "httpPost")]
        HttpPost,
    }

    #[derive(Deserialize, Serialize)]
    pub struct ServerDocument {
        pub r#type: ServerDocumentType,
    }

    #[derive(Deserialize, Serialize)]
    pub struct FirestoreServiceAccount {
        pub project_id: String,
    }
}
