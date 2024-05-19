use std::{env::set_var, path::Path};
use firestore::{errors::FirestoreError, FirestoreDb};
use futures::{stream::BoxStream, StreamExt};

use crate::models::ServerDocument;

const FIREBASE_CREDENTIALS: &str = "./firebase_reader.json";

fn configure_credentials() {
    if Path::new(FIREBASE_CREDENTIALS).exists() {
        set_var("GOOGLE_APPLICATION_CREDENTIALS", FIREBASE_CREDENTIALS)
    }
}

pub async fn create() -> Result<FirestoreDb, FirestoreError> {
    configure_credentials();

    let firestore_db = FirestoreDb::new("sesamo-iot").await?;

    let objs_stream: BoxStream<ServerDocument> = firestore_db
        .fluent()
        .list()
        .from("servers")
        .obj()
        .stream_all()
        .await?;

    let as_vec: Vec<ServerDocument> = objs_stream.collect().await;
    println!("{:?}", as_vec);



    let db = Firestore::new();

    Ok(firestore_db)
}