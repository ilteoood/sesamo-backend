use crate::firebase::get_firestore_instance;
use actix_web::guard::GuardContext;
use futures::executor::block_on;

use crate::models::OpenRequest;

pub fn can_open(guard: &GuardContext) -> bool {
    let firebase_db = block_on(get_firestore_instance());

    let binding = guard.req_data();
    let request: Option<&OpenRequest> = binding.get();

    let object: Option<&str> = guard.head().uri.path().split('/').last();

    if let (Some(request), Some(object)) = (request, object) {
        return firebase_db.server_exists(&request.server_id)
            && firebase_db.check_configuration(&request.server_id, object)
            && firebase_db.has_device_access(&request.server_id, &request.device_id);
    }

    false
}
