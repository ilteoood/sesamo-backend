use crate::firebase::Firestore;
use actix_web::guard::GuardContext;
use futures::executor::block_on;

use crate::models::OpenRequest;

pub fn can_open(guard: &GuardContext) -> bool {
    let firebase_db = Firestore::new();
    let firebase_db = block_on(firebase_db);

    let binding = guard.req_data();
    let request: Option<&OpenRequest> = binding.get();


    if let (Ok(firebase_db), Some(request)) = (firebase_db, request) {
        return firebase_db.server_exists(&request.server_id)
            && firebase_db.check_configuration(&request.server_id, "object", )
            && firebase_db.has_device_access(&request.server_id, &request.device_id);
    }

    false
}

#[cfg(test)]
mod test {
    use crate::firebase::MockFirestore;

    use super::*;
    use actix_web::test;

    #[test]
    async fn test_can_open() {
        /*
            let mock_firestore = MockFirestore::new().await;
            mock_firestore
                .expect_server_exists("server_id".to_string())
                .returning(true);
            mock_firestore
                .expect_check_configuration("object".to_string())
                .returning(true);
            mock_firestore
                .expect_has_device_access("device_id".to_string())
                .returning(true);

        assert!(can_open());
        */
    }
}
