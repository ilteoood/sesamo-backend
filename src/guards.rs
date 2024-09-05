use crate::{firebase::get_firestore_instance, models::MessageResponse};
use actix_web::{web::Path, HttpResponse};

use crate::models::OpenRequest;

pub async fn can_open_guard(
    request: &OpenRequest,
    object: &Path<String>,
) -> Result<(), HttpResponse> {
    let firebase_db = get_firestore_instance().await;

    if !firebase_db.server_exists(&request.server_id) {
        return Err(HttpResponse::InternalServerError().json(MessageResponse {
            message_id: String::from("invalid_server"),
        }));
    }

    if !firebase_db.check_configuration(&request.server_id, object.as_str()) {
        return Err(HttpResponse::InternalServerError().json(MessageResponse {
            message_id: String::from("invalid_action"),
        }));
    }

    if !firebase_db.has_device_access(request.server_id.as_str(), request.device_id.as_str()) {
        return Err(HttpResponse::InternalServerError().json(MessageResponse {
            message_id: String::from("unauthorized_device"),
        }));
    }

    Ok(())
}
