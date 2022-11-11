use std::collections::HashMap;

use anyhow::{Result, anyhow};
use serenity::{prelude::TypeMapKey, model::{prelude::UserId, user::OnlineStatus}};
use tokio::time::Instant;

use crate::db::data::SleepState;

pub struct PresenceCache;

impl TypeMapKey for PresenceCache {
    type Value = HashMap<UserId, PresenceCacheEntry>;
}

pub fn status_to_sleep(status: OnlineStatus) -> Result<SleepState> {
    match status {
        OnlineStatus::DoNotDisturb | OnlineStatus::Online => Ok(SleepState::Woke),
        OnlineStatus::Idle | OnlineStatus::Offline | OnlineStatus::Invisible => Ok(SleepState::Sleep),
        _ => Err(anyhow!("Unknown presence status: {:?}", status)),
    }
}

pub struct PresenceCacheEntry {
    pub state: SleepState,
    pub instant: Instant,
}

impl PresenceCacheEntry {
    pub fn new(state: SleepState) -> Self {
        Self {
            state: state,
            instant: Instant::now(),
        }
    }
}