pub mod woke;
pub mod sleep;

use chrono::Utc;
use mongodb::Database;
use serenity::{prelude::Context, model::prelude::{interaction::application_command::ApplicationCommandInteraction}};
use crate::db;
use anyhow::Result;

use crate::{util::{
    seconds_pretty,
    interaction,
}, db::data::{Sleep, SleepState}};

fn error(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "You are already awake silly!",
        SleepState::Sleep => "You are already in the bed silly!",
    }
}

pub fn title(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "Good Morning!",
        SleepState::Sleep => "Good Night~",
    }
}

pub fn description(state: SleepState, time: i64) -> String {
    match state {
        SleepState::Woke => format!("You were asleep for {}", seconds_pretty(time)),
        SleepState::Sleep => format!("You were awake for {}", seconds_pretty(time)),
    }
}

pub fn footer(state: SleepState, sleep_last: &Sleep) -> String {
    if sleep_last.mentions.is_empty() {
        String::new()
    } else {
        let len = sleep_last.mentions.len();

        match state {
            SleepState::Woke => format!("They finally woke up after {} people begging them to", len),
            SleepState::Sleep => format!("They finally went to sleep after {} people begging them to", len),
        }
    }
}

pub async fn persist(db: &Database, sleep: Sleep, prev: Option<(&Sleep, i64)>) -> Result<()> {
    if let Some((sleep_last, awake_time)) = prev {
        db::update_sleep_time(db, sleep_last._id, awake_time).await?;
    }

    db::insert_sleep(db, sleep).await?;

    Ok(())
}

pub async fn run(state: SleepState, db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let id = command.user.id.to_string();

    let sleep_last = db::fetch_sleep_last(db, &id).await?;
    let sleep = Sleep::new(id, state, Utc::now());

    let title = title(state);

    let mut embed = super::sleep_embed(state);
    
    embed.title(title);

    if let Some(sleep_last) = sleep_last {
        if sleep_last.state == state {
            let error = error(state);

            interaction::send_silent(ctx, command, error).await;
        } else {
            let now = Utc::now();
            let time = super::calc_time(&sleep_last, now);

            persist(db, sleep, Some((&sleep_last, time))).await?;

            let description = description(state, time);
            let footer_text = footer(state, &sleep_last);

            embed
                .description(description)
                .footer(|footer| {
                    footer
                        .text(footer_text)
                });

            interaction::send(ctx, command, |message| {
                message
                    .set_embed(embed)
            }).await;
        }
    } else {
        persist(db, sleep, None).await?;

        interaction::send(ctx, command, |message| {
            message
                .set_embed(embed)
        }).await;
    }
    
    Ok(())
}