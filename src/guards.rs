use crate::{firebase::get_firestore_instance, models::MessageResponse};
use actix_web::web::Path;

use crate::models::OpenRequest;

pub async fn can_open_guard(
    request: &OpenRequest,
    object: &Path<String>,
) -> Result<(), MessageResponse> {
    let firebase_db = get_firestore_instance().await;

    if !firebase_db.server_exists(&request.server_id) {
        return Err(MessageResponse {
            message_id: String::from("invalid_server"),
        });
    }

    if !firebase_db.check_configuration(&request.server_id, object.as_str()) {
        return Err(MessageResponse {
            message_id: String::from("invalid_action"),
        });
    }

    if !firebase_db.has_device_access(request.server_id.as_str(), request.device_id.as_str()) {
        return Err(MessageResponse {
            message_id: String::from("unauthorized_device"),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use actix_web::test;

    async fn invoke_guard(
        server_id: &str,
        device_id: &str,
        object: &str,
    ) -> Result<(), MessageResponse> {
        env::set_var("FIRESTORE_DATABASE", "test");
        can_open_guard(
            &OpenRequest {
                server_id: server_id.to_string(),
                device_id: device_id.to_string(),
            },
            &Path::from(object.to_owned()),
        )
        .await
    }

    #[test]
    async fn test_server_not_exists() {
        let response = invoke_guard("test", "test_device", "gate").await;

        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            MessageResponse {
                message_id: String::from("invalid_server")
            }
        );
    }

    #[test]
    async fn test_check_configuration() {
        let response = invoke_guard("test_server", "test_device", "test").await;
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            MessageResponse {
                message_id: String::from("invalid_action")
            }
        );
    }

    #[test]
    async fn test_device_access() {
        let response = invoke_guard("test_server", "test", "gate").await;
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err(),
            MessageResponse {
                message_id: String::from("unauthorized_device")
            }
        );
    }

    #[test]
    async fn test_ok() {
        let response = invoke_guard("test_server", "test_device", "gate").await;
        assert!(response.is_ok());
    }
}
