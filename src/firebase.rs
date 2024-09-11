use crate::models::firebase::{
    FirestoreServiceAccount, ObjectConfiguration, ServerAllowedDevices, ServerDocument,
    ServerDocumentType,
};
use firestore::{
    errors::FirestoreError, FirestoreCache, FirestoreCacheCollectionConfiguration,
    FirestoreCacheCollectionLoadMode, FirestoreCacheConfiguration, FirestoreDb, FirestoreDbOptions,
    FirestoreListenerTarget, FirestoreMemListenStateStorage, FirestoreMemoryCacheBackend,
    FirestoreTempFilesListenStateStorage, ParentPathBuilder, FIREBASE_DEFAULT_DATABASE_ID,
};
use std::{
    env::{self, set_var},
    error::Error,
    fs::File,
    io::BufReader,
    path::Path,
};
use tokio::sync::OnceCell;

const SERVERS_COLLECTION: &str = "servers";
const CONFIGURATIONS_COLLECTION: &str = "configurations";
const ALLOWED_DEVICES_COLLECTION: &str = "allowedDevices";
const GOOGLE_APPLICATION_CREDENTIALS: &str = "GOOGLE_APPLICATION_CREDENTIALS";
const PROJECT_ID: &str = "GOOGLE_CLOUD_PROJECT";

pub struct Firestore {
    firestore_db: FirestoreDb,
    cache: FirestoreCache<FirestoreMemoryCacheBackend, FirestoreMemListenStateStorage>,
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

        let firestore_options = FirestoreDbOptions::new(env::var(PROJECT_ID)?).with_database_id(
            env::var("FIRESTORE_DATABASE").unwrap_or(String::from(FIREBASE_DEFAULT_DATABASE_ID)),
        );

        let firestore_db = FirestoreDb::with_options(firestore_options).await?;

        let mut listener = firestore_db
            .create_listener(FirestoreTempFilesListenStateStorage::new())
            .await?;

        firestore_db
            .fluent()
            .select()
            .from(SERVERS_COLLECTION)
            .listen()
            .add_target(TARGET_SERVERS, &mut listener)?;

        let mut cache = FirestoreCache::new(
            SERVERS_COLLECTION.into(),
            &firestore_db,
            FirestoreMemoryCacheBackend::new(
                FirestoreCacheConfiguration::new().add_collection_config(
                    &firestore_db,
                    FirestoreCacheCollectionConfiguration::new(
                        SERVERS_COLLECTION,
                        FirestoreListenerTarget::new(1000),
                        FirestoreCacheCollectionLoadMode::PreloadNone,
                    ),
                ),
            )?,
            FirestoreMemListenStateStorage::new(),
        )
        .await?;

        cache.load().await?;

        Ok(Self {
            firestore_db,
            cache,
        })
    }

    async fn read_server(&self, server_id: &str) -> Result<Option<ServerDocument>, FirestoreError> {
        self.firestore_db
            .read_through_cache(&self.cache)
            .fluent()
            .select()
            .by_id_in(SERVERS_COLLECTION)
            .obj()
            .one(server_id)
            .await
    }

    pub async fn server_exists(&self, server_id: &str) -> bool {
        let server = self.read_server(server_id).await;

        server.is_ok() && server.unwrap().is_some()
    }

    pub async fn get_server_type(&self, server_id: &str) -> ServerDocumentType {
        let server = self.read_server(server_id).await.unwrap().unwrap();

        server.r#type
    }

    fn parent_path_builder(&self, server_id: &str) -> ParentPathBuilder {
        self.firestore_db
            .parent_path(SERVERS_COLLECTION, server_id)
            .unwrap()
    }

    async fn retrieve_configurations(
        &self,
        server_id: &str,
        object: &str,
    ) -> Result<Option<ObjectConfiguration>, FirestoreError> {
        let parent_path = self.parent_path_builder(server_id);

        self.firestore_db
            .read_through_cache(&self.cache)
            .fluent()
            .select()
            .by_id_in(CONFIGURATIONS_COLLECTION)
            .parent(parent_path)
            .obj()
            .one(object)
            .await
    }

    pub async fn check_configuration(&self, server_id: &str, object: &str) -> bool {
        let configuration = self.retrieve_configurations(server_id, object).await;

        configuration.is_ok() && configuration.unwrap().is_some()
    }

    pub async fn get_object_configuration(
        &self,
        server_id: &str,
        object: &str,
    ) -> ObjectConfiguration {
        let configuration = self.retrieve_configurations(server_id, object).await;

        configuration.unwrap().unwrap()
    }

    pub async fn has_device_access(&self, server_id: &str, device_id: &str) -> bool {
        let parent_path = self.parent_path_builder(server_id);

        let query_result: Result<Option<ServerAllowedDevices>, FirestoreError> = self
            .firestore_db
            .read_through_cache(&self.cache)
            .fluent()
            .select()
            .by_id_in(CONFIGURATIONS_COLLECTION)
            .parent(parent_path)
            .obj()
            .one(ALLOWED_DEVICES_COLLECTION)
            .await;

        let allowed_devices: ServerAllowedDevices = query_result
            .unwrap_or(Some(ServerAllowedDevices::default()))
            .unwrap_or(ServerAllowedDevices::default());

        allowed_devices.list.contains(&String::from(device_id))
    }

    fn configure_credentials() {
        let firebase_credentials = "./firebase_reader.json";
        if Path::new(firebase_credentials).exists() {
            set_var(GOOGLE_APPLICATION_CREDENTIALS, firebase_credentials);

            let service_account = Self::read_service_account().unwrap();
            set_var(PROJECT_ID, service_account.project_id);
        }
    }

    fn read_service_account() -> Result<FirestoreServiceAccount, Box<dyn Error>> {
        let file = File::open(env::var(GOOGLE_APPLICATION_CREDENTIALS)?)?;
        let reader = BufReader::new(file);

        Ok(serde_json::from_reader(reader).unwrap())
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
