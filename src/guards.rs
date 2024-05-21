use crate::firebase::Firestore;
use actix_web::guard::GuardContext;
use futures::executor::block_on;

pub fn can_open(_guard: &GuardContext) -> bool {
    let firebase_db = Firestore::new();
    let result = block_on(firebase_db);

    if let Ok(instance) = result {
        return instance.server_exists("server_id".to_string())
            && instance.check_configuration("object".to_string())
            && instance.has_device_access("device_id".to_string());
    }

    false
}
