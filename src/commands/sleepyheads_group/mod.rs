use std::{collections::HashMap, cmp::Ordering};

use anyhow::Result;
use chrono::Utc;
use mongodb::Database;
use serenity::{prelude::Context, model::prelude::{GuildId, UserId, Member, interaction::application_command::ApplicationCommandInteraction}};

use crate::db;
use crate::db::data::{SleepState, Sleep};
use crate::util::{
    interaction,
    seconds_pretty,
};

pub mod sleepyheads;
pub mod wakers;

fn get_members(ctx: &Context, guild_id: Option<GuildId>) -> Option<HashMap<UserId, Member>> {
    let guild_id = guild_id?;

    ctx.cache.guild_field(guild_id, |guild| {
        guild
            .members.clone()
    })
}

async fn fetch(state: SleepState, db: &Database, members: HashMap<UserId, Member>) -> Result<Vec<Sleep>> {
    let now = Utc::now();

    let mut sleep_list = Vec::new();

    for (user_id, _) in members {
        let id = user_id.to_string();
        let sleep = db::fetch_sleep_last(db, &id).await?;

        if let Some(mut sleep) = sleep {
            if state == sleep.state {
                sleep.time = super::calc_time(&sleep, now);

                sleep_list.push(sleep);
            }
        }
    }

    Ok(sleep_list)
}

fn sort(sleep_list: &mut Vec<Sleep>) {
    sleep_list.sort_unstable_by(|a, b| {
        if a.time == b.time {
            Ordering::Equal
        } else if a.time > b.time {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })
}

fn empty_list(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "It seems that no one is awake",
        SleepState::Sleep => "It seems that no one is asleep",
    }
}

fn title(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "Wakers",
        SleepState::Sleep => "Sleepyheads",
    }
}

fn description(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "Wakers ranked by awake time",
        SleepState::Sleep => "Sleepyheads ranked by sleep time",
    }
}

fn thumbnail(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "https://media.tenor.com/0WwPx5h_mZYAAAAM/gintoki-no-sleep.gif",
        SleepState::Sleep => "https://media.tenor.com/jrBQmuo2elMAAAAM/anime-sleep.gif",
    }
}

fn fields(sleep_list: Vec<Sleep>) -> Vec<(String, String, bool)> {
    let mut fields = Vec::new();

    for (i, sleep) in sleep_list.iter().enumerate() {
        let field = (
            (i + 1).to_string(),
            format!("<@{}> : {}", sleep.id, seconds_pretty(sleep.time)),
            false
        );

        fields.push(field);
    }

    fields
}

pub async fn run(state: SleepState, db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let members = get_members(ctx, command.guild_id);

    match members {
        Some(members) => {
            let mut sleep_list = fetch(state, db, members).await?;

            if sleep_list.is_empty() {
                let content = empty_list(state);

                interaction::send_silent(ctx, command, content).await;
            }

            sort(&mut sleep_list);

            let mut embed = super::colorful_embed();

            let title = title(state);
            let description = description(state);
            let thumbnail = thumbnail(state);
            let fields = fields(sleep_list);

            embed
                .title(title)
                .description(description)
                .thumbnail(thumbnail)
                .fields(fields);
            
            interaction::send(ctx, command, |message| {
                message
                    .set_embed(embed)
            }).await;
        }
        None => {
            interaction::send_silent(ctx, command, "Failed to acquire guild info").await;
        }
    }

    Ok(())
}