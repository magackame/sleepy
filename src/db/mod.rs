use mongodb::Cursor;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{InsertOneResult, UpdateResult, DeleteResult};
use mongodb::{
    bson::doc,
    Client,
    Database,
};
use mongodb::options::{
    ClientOptions,
    FindOneOptions,
    FindOptions,
};

use anyhow::Result;

pub mod data;
use data::{
    Sleep,
    UserOptions
};

const DB_NAME: &str = "sleepy";
const SLEEP_COLLECTION_NAME: &str = "sleep";
const USER_COLLECTION_NAME: &str = "user";

pub async fn connect() -> Database {
    let options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .expect("Failed to parse db connection string");
    
    let client = Client::with_options(options)
        .expect("Failed to connect to db");
    
    client.database(DB_NAME)
}

pub async fn fetch_sleep_last(db: &Database, id: &str) -> Result<Option<Sleep>> {
    let collection = db.collection::<Sleep>(SLEEP_COLLECTION_NAME);

    let filter = doc! { "id": id };
    let options = FindOneOptions::builder()
        .sort(doc! { "_id": -1 })
        .build();

    let sleep = collection.find_one(filter, options).await?;

    Ok(sleep)
}

pub async fn insert_sleep(db: &Database, sleep: Sleep) -> Result<InsertOneResult> {
    let collection = db.collection::<Sleep>(SLEEP_COLLECTION_NAME);

    let insert = collection.insert_one(sleep, None).await?;

    Ok(insert)
}

pub async fn update_sleep_time(db: &Database, _id: ObjectId, time: i64) -> Result<UpdateResult> {
    let collection = db.collection::<Sleep>(SLEEP_COLLECTION_NAME);

    let query = doc! { "_id": _id };
    let update = doc! { "$set" : { "time": time } };

    let update = collection.update_one(query, update, None).await?;

    Ok(update)
}

pub async fn fetch_sleep_all(db: &Database, id: &str) -> Result<Cursor<Sleep>> {
    const FETCH_LIMIT: i64 = 60_000;
    
    let collection = db.collection::<Sleep>(SLEEP_COLLECTION_NAME);

    let filter = doc! { "id": id };
    let options = FindOptions::builder()
        .sort(doc! { "_id": -1 })
        .limit(FETCH_LIMIT)
        .build();

    let cursor = collection.find(filter, options).await?;
    
    Ok(cursor)
}

pub async fn delete_sleep_all(db: &Database, id: &str) -> Result<DeleteResult> {
    let collection = db.collection::<Sleep>(SLEEP_COLLECTION_NAME);

    let filter = doc! { "id": id };

    let delete = collection.delete_many(filter, None).await?;
    
    Ok(delete)
}

pub async fn update_sleep_mentions(db: &Database, _id: ObjectId, sender_id: &str) -> Result<UpdateResult> {
    let collection = db.collection::<Sleep>(SLEEP_COLLECTION_NAME);

    let query = doc! { "_id": _id };
    let update = doc! { "$push" : { "mentions": sender_id } };

    let update = collection.update_one(query, update, None).await?;

    Ok(update)
}

pub async fn fetch_user(db: &Database, id: &str) -> Result<Option<UserOptions>> {
    let collection = db.collection::<UserOptions>(USER_COLLECTION_NAME);

    let filter = doc! { "id": id };

    let user = collection.find_one(filter, None).await?;

    Ok(user)
}

pub async fn insert_user(db: &Database, user: UserOptions) -> Result<InsertOneResult> {
    let collection = db.collection::<UserOptions>(USER_COLLECTION_NAME);

    let insert = collection.insert_one(user, None).await?;

    Ok(insert)
}

pub async fn update_user_autowoke(db: &Database, id: &str, autowoke: bool) -> Result<UpdateResult> {
    let collection = db.collection::<UserOptions>(USER_COLLECTION_NAME);

    let query = doc! { "id": id };
    let update = doc! { "$set" : { "autowoke": autowoke } };

    let update = collection.update_one(query, update, None).await?;

    Ok(update)
}

pub async fn update_user_autosleep(db: &Database, id: &str, autosleep: Option<i64>) -> Result<UpdateResult> {
    let collection = db.collection::<UserOptions>(USER_COLLECTION_NAME);

    let query = doc! { "id": id };
    let update = doc! { "$set" : { "autosleep": autosleep } };

    let update = collection.update_one(query, update, None).await?;

    Ok(update)
}