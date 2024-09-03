use crate::models::firebase::{
    FirestoreServiceAccount, ObjectRequest, ServerAllowedDevices, ServerDocument,
    ServerDocumentBase, ServerDocumentConfiguration,
};
use firestore::{
    FirestoreDb, FirestoreListenEvent, FirestoreListenerTarget,
    FirestoreTempFilesListenStateStorage,
};
use futures::{future, StreamExt};
use std::{
    collections::HashMap,
    env::set_var,
    error::Error,
    fs::File,
    io::BufReader,
    path::Path,
    sync::{Arc, RwLock},
};
use tokio::sync::OnceCell;

const FIREBASE_CREDENTIALS: &str = "./firebase_reader.json";

const SERVERS_COLLECTION: &str = "servers";
const CONFIGURATIONS_COLLECTION: &str = "configurations";
const ALLOWED_DEVICES_COLLECTION: &str = "allowedDevices";

pub struct Firestore {
    server_map: Arc<RwLock<HashMap<String, ServerDocument>>>,
}

const TARGET_SERVERS: FirestoreListenerTarget = FirestoreListenerTarget::new(94_u32);

static ONCE: OnceCell<Firestore> = OnceCell::const_new();

pub async fn get_firestore_instance() -> &'static Firestore {
    ONCE.get_or_init(|| async { Firestore::new().await.unwrap() })
        .await
}

impl Firestore {
    pub async fn new() -> Result<Firestore, Box<dyn Error>> {
        Self::configure_credentials();

        let firestore_service_account = Self::read_service_account()?;

        let firestore_db = FirestoreDb::new(firestore_service_account.project_id.as_str()).await?;

        let server_map = Self::build_server_map(&firestore_db).await?;

        let mut listener = firestore_db
            .create_listener(FirestoreTempFilesListenStateStorage::new())
            .await?;

        firestore_db
            .fluent()
            .select()
            .from(SERVERS_COLLECTION)
            .listen()
            .add_target(TARGET_SERVERS, &mut listener)?;

        let server_map_lock = Arc::new(RwLock::new(server_map));
        let server_map_clone = Arc::clone(&server_map_lock.clone());

        listener
            .start(move |event| {
                let value = firestore_db.clone();
                let server_map_lock = Arc::clone(&server_map_lock.clone());

                async move {
                    match event {
                        FirestoreListenEvent::DocumentChange(ref doc_change) => {
                            if let Some(doc) = &doc_change.document {
                                let doc: ServerDocumentBase =
                                    FirestoreDb::deserialize_doc_to::<ServerDocumentBase>(doc)
                                        .expect("Deserialized object");
                                let doc_enriched = Self::enrich_document(&value, doc).await;

                                let mut server_map =
                                    server_map_lock.write().expect("Poisoned lock");
                                server_map.insert(doc_enriched.id.clone(), doc_enriched);
                            }
                        }
                        _ => {
                            println!("Received a listen response event to handle: {event:?}");
                        }
                    }
                    Ok(())
                }
            })
            .await?;

        Ok(Self {
            server_map: server_map_clone,
        })
    }

    pub fn server_exists(&self, server_id: &str) -> bool {
        self.server_map.read().unwrap().contains_key(server_id)
    }

    pub fn check_configuration(&self, server_id: &str, object: &str) -> bool {
        self.server_map
            .read()
            .unwrap()
            .get(server_id)
            .unwrap()
            .configurations
            .objects
            .contains_key(object)
    }

    pub fn has_device_access(&self, server_id: &str, device_id: &str) -> bool {
        self.server_map
            .read()
            .unwrap()
            .get(server_id)
            .unwrap()
            .configurations
            .allowed_devices
            .list
            .contains(&String::from(device_id))
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

    async fn enrich_document(
        firestore_db: &FirestoreDb,
        doc: ServerDocumentBase,
    ) -> ServerDocument {
        let parent_path = firestore_db
            .parent_path(SERVERS_COLLECTION, doc.id.clone())
            .unwrap();

        let allowed_devices: ServerAllowedDevices = firestore_db
            .fluent()
            .select()
            .by_id_in(CONFIGURATIONS_COLLECTION)
            .parent(&parent_path)
            .obj()
            .one(ALLOWED_DEVICES_COLLECTION)
            .await
            .unwrap()
            .unwrap();

        let objects = firestore_db
            .fluent()
            .list()
            .from(CONFIGURATIONS_COLLECTION)
            .parent(&parent_path)
            .obj()
            .stream_all()
            .await
            .unwrap()
            .collect::<Vec<ObjectRequest>>()
            .await
            .into_iter()
            .map(|doc| (doc.id.clone(), doc))
            .collect();

        let configurations = ServerDocumentConfiguration {
            allowed_devices,
            objects,
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
