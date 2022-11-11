use std::{time::Duration, ops::Sub};

use anyhow::{Result, anyhow};
use chrono::{Utc, DateTime};
use mongodb::Database;
use serenity::{model::{user::User, prelude::UserId}, prelude::Context};
use tokio::time::Instant;
use crate::db::{self, data::{SleepState, Sleep, UserOptions}};
use crate::commands::{
    calc_time,
    sleep_embed,
    sleep_group::{
        title,
        description,
        footer,
        persist,
    },
};

pub mod cache;
use cache::PresenceCache;

fn get_user_from_cache(user_id: UserId, ctx: &Context) -> Result<User> {
    match ctx.cache.user(user_id) {
        Some(user) => Ok(user),
        None => Err(anyhow!("User not found in cache")),
    }
}

const SECS_IN_HOUR: u64 = 3600;
const UPDATE_DELAY_SECS: u64 = 1 * SECS_IN_HOUR;

pub fn start_presence_worker(db: Database, ctx: Context) -> tokio::task::JoinHandle<Result<()>> {
    tokio::spawn(async move {
        loop {
            let read_lock = ctx.data.read().await;
            let presence_cache = read_lock.get::<PresenceCache>()
                .expect("Expected PresenceCache in ctx.data TypeMap");

            let now = Instant::now();
            for (user_id, entry) in presence_cache {
                if entry.state == SleepState::Sleep {
                    let id = user_id.to_string();
                    let hours_since_offline = (now - entry.instant).as_secs() / SECS_IN_HOUR;

                    // TODO: Remove double fetch in presence_handler
                    let user_options = db::fetch_user(&db, &id).await?;

                    if let Some(user_options) = user_options {
                        if let Some(hours) = user_options.autosleep {
                            if hours_since_offline >= hours as u64 {
                                let date = Utc::now().sub(chrono::Duration::hours(hours_since_offline as i64));

                                if let Err(why) = presence_handler(&db, &ctx, *user_id, entry.state, date).await {
                                    println!("Error in presence worker: {}", why);
                                }
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(UPDATE_DELAY_SECS)).await;
        }
    })
}

fn continue_handling(presence_state: SleepState, user_options: UserOptions) -> bool {
    if user_options.autowoke && presence_state == SleepState::Woke {
        true
    } else if user_options.autosleep.is_some() && presence_state == SleepState::Sleep {
        true
    } else {
        false
    }
}

pub async fn presence_handler(db: &Database, ctx: &Context, user_id: UserId, presence_state: SleepState, date: DateTime<Utc>) -> Result<()> {
    let user = get_user_from_cache(user_id, ctx)?;
    let id = user.id.to_string();
    let user_options = db::fetch_user(db, &id).await?;

    let mut embed = sleep_embed(presence_state);

    let title = title(presence_state);

    embed.title(title);

    if let Some(user_options) = user_options {
        if continue_handling(presence_state, user_options) {
            let sleep_last = db::fetch_sleep_last(db, &id).await?;
            let sleep = Sleep::new(id, presence_state, date);

            match sleep_last {
                Some(sleep_last) => {
                    if sleep_last.state != presence_state {
                        let now = Utc::now();
                        let time = calc_time(&sleep_last, now);
    
                        let description = description(presence_state, time);
                        let footer_text = footer(presence_state, &sleep_last);

                        persist(db, sleep, Some((&sleep_last, time))).await?;

                        if presence_state == SleepState::Woke {
                            embed
                            .description(description)
                            .footer(|footer| {
                                footer
                                    .text(footer_text)
                            });
    
                            user.direct_message(&ctx.http, |message| {
                                message
                                    .set_embed(embed)
                            }).await?;
                        }
                    }
                }
                None => {
                    persist(db, sleep, None).await?;
    
                    if presence_state == SleepState::Woke {
                        user.direct_message(&ctx.http, |message| {
                            message
                                .set_embed(embed)
                        }).await?;
                    }
                }
            }
        }
    }

    Ok(())
}