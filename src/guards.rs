use crate::firebase::Firestore;
use actix_web::guard::GuardContext;
use futures::executor::block_on;

pub fn can_open(_guard: &GuardContext) -> bool {
    let firebase_db = Firestore::new();
    let result = block_on(firebase_db);

    if let Ok(instance) = result {
        return instance.server_exists("server_id")
            && instance.check_configuration("server_id", "object", )
            && instance.has_device_access("server_id", "device_id");
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
