use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum SleepState {
    Woke,
    Sleep
}

#[derive(Deserialize, Serialize)]
pub struct Sleep {
    pub _id: ObjectId,
    pub id: String,
    pub state: SleepState,
    pub time: i64,
    pub mentions: Vec<String>,
    pub date: DateTime<Utc>,
}

impl Sleep {
    pub fn new(id: String, state: SleepState, date: DateTime<Utc>) -> Self {
        Self {
            _id: ObjectId::new(),
            id: id,
            state: state,
            time: 0,
            mentions: Vec::new(),
            date: date,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserOptions {
    pub _id: ObjectId,
    pub id: String,
    pub autowoke: bool,
    pub autosleep: Option<i64>,
}

impl UserOptions {
    pub fn new(id: String) -> Self {
        Self {
            _id: ObjectId::new(),
            id: id,
            autowoke: false,
            autosleep: None,
        }
    }
}