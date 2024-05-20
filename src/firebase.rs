use firestore::FirestoreDb;
use futures::{future, StreamExt};
use std::{collections::HashMap, env::set_var, error::Error, fs::File, io::BufReader, path::Path};

use crate::models::firebase::{
    FirestoreServiceAccount, ObjectRequest, ServerDocument, ServerDocumentBase, ServerDocumentConfiguration, ServerAllowedDevices
};

const FIREBASE_CREDENTIALS: &str =
    "/Users/ilteoood/Documents/git/personal/sesamo-backend/firebase_reader.json";

const SERVERS_COLLECTION: &str = "servers";
const CONFIGURATIONS_COLLECTION: &str = "configurations";

struct Firestore {
    firestore_db: FirestoreDb,
    server_map: HashMap<String, ServerDocument>,
}

impl Firestore {
    async fn new() -> Result<Firestore, Box<dyn Error>> {
        Self::configure_credentials();

        let firestore_service_account = Self::read_service_account()?;

        let firestore_db = FirestoreDb::new(firestore_service_account.project_id.as_str()).await?;

        let server_map = Self::build_server_map(&firestore_db).await?;

        Ok(Self {
            firestore_db,
            server_map,
        })
    }

    fn configure_credentials() {
        if Path::new(FIREBASE_CREDENTIALS).exists() {
            set_var("GOOGLE_APPLICATION_CREDENTIALS", FIREBASE_CREDENTIALS)
        }
    }

    fn read_service_account() -> Result<FirestoreServiceAccount, Box<dyn Error>> {
        let file = File::open(FIREBASE_CREDENTIALS)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader).unwrap())
    }

    async fn build_server_map(
        firestore_db: &FirestoreDb,
    ) -> Result<HashMap<String, ServerDocument>, Box<dyn Error>> {
        let server_documents: Vec<ServerDocumentBase> = firestore_db
            .fluent()
            .list()
            .from(SERVERS_COLLECTION)
            .obj()
            .stream_all()
            .await?
            .collect()
            .await;

        let servers: Vec<ServerDocument> = future::join_all(
            server_documents
                .into_iter()
                .map(|doc| Self::enrich_document(firestore_db, doc)),
        )
        .await;

        let server_map = servers
            .into_iter()
            .map(|doc| (doc.id.clone(), doc))
            .collect();

        Ok(server_map)
    }

    async fn enrich_document(firestore_db: &FirestoreDb, doc: ServerDocumentBase) -> ServerDocument {
        let parent_path = firestore_db
            .parent_path(SERVERS_COLLECTION, doc.id.clone())
            .unwrap();

        let allowed_devices: ServerAllowedDevices = firestore_db
            .fluent()
            .select()
            .by_id_in(CONFIGURATIONS_COLLECTION)
            .parent(&parent_path)
            .obj()
            .one("allowedDevices")
            .await
            .unwrap()
            .unwrap();

        let gate: ObjectRequest = firestore_db
            .fluent()
            .select()
            .by_id_in(CONFIGURATIONS_COLLECTION)
            .parent(&parent_path)
            .obj()
            .one("gate")
            .await
            .unwrap()
            .unwrap();

        let wicket: ObjectRequest = firestore_db
            .fluent()
            .select()
            .by_id_in(CONFIGURATIONS_COLLECTION)
            .parent(&parent_path)
            .obj()
            .one("wicket")
            .await
            .unwrap()
            .unwrap();

        let configurations = ServerDocumentConfiguration {
            allowed_devices,
            gate,
            wicket,
        };

        ServerDocument {
            configurations,
            id: doc.id,
            name: doc.name,
            r#type: doc.r#type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[test]
    async fn test_configure_credentials() {
        let result = Firestore::new().await;

        assert!(result.is_ok());
    }
}
