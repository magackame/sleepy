use anyhow::Result;
use mongodb::Database;
use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};

use crate::db;
use crate::db::data::{SleepState, Sleep};
use crate::util::{
    interaction,
    option::ParsedOptions,
};

pub mod wakey;
pub mod nap;

fn error(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "That person is already awake silly!",
        SleepState::Sleep => "That person is already asleep silly!",
    }
}

fn content(receiver_id: &str) -> String {
    format!("<@{}>", receiver_id)
}

fn title(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "WAKE UP",
        SleepState::Sleep => "GO TO SLEEP",
    }
}

fn description(state: SleepState, application_id: &str, sender_id: &str, receiver_id: &str) -> String {
    if receiver_id == sender_id {
        match state {
            SleepState::Woke => "Hey, you are already awake\nAt least as far as I'm concerned...".to_string(),
            SleepState::Sleep => format!("Somebody help <@{}>\nThey are so tired, they basically\nbeg themselves to go to sleep", sender_id),
        }
    } else if receiver_id == application_id {
        match state {
            SleepState::Woke => "Welp, actually,\nI never sleep, so...".to_string(),
            SleepState::Sleep => "If I'm going to sleep, who is going\nto watch over all of you sleepyheads?".to_string(),
        }
    } else {
        match state {
            SleepState::Woke => format!("Wake up <@{}> you sleepyhead~", receiver_id),
            SleepState::Sleep => format!("The day is already over\nGo get some rest <@{}> ~", receiver_id),
        }
    }
}

async fn persist(db: &Database, receiver_sleep_last: &Sleep, sender_id: &String, receiver_id: &str) -> Result<()> {
    if !receiver_sleep_last.mentions.contains(sender_id) && sender_id != receiver_id {
        db::update_sleep_mentions(db, receiver_sleep_last._id, sender_id).await?;
    }

    Ok(())
}

const USER_OPTION_NAME: &str = "user";

pub async fn run(state: SleepState, db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let sender_id = command.user.id.to_string();
    let application_id = ctx.cache.current_user_id().to_string();

    let parsed_options = ParsedOptions::parse(&command.data.options);

    let receiver_id = parsed_options.get_user(USER_OPTION_NAME)?.id.to_string();

    let receiver_sleep_last = db::fetch_sleep_last(db, &receiver_id).await?;

    let content = content(&receiver_id);
    let title = title(state);
    let description = description(state, &application_id, &sender_id, &receiver_id);

    let mut embed = super::nap_embed(state);

    embed
        .title(title)
        .description(description);

    match receiver_sleep_last {
        Some(receiver_sleep_last) => {
            if receiver_sleep_last.state == state {
                let error = error(state);

                interaction::send_silent(ctx, command, error).await;
            } else {
                persist(db, &receiver_sleep_last, &sender_id, &receiver_id).await?;

                interaction::send(ctx, command, |message| {
                    message
                        .content(content)
                        .set_embed(embed)
                }).await;
            }
        }
        None => {
            interaction::send(ctx, command, |message| {
                message
                    .content(content)
                    .set_embed(embed)
            }).await;
        }
    }
    
    Ok(())
}