use serenity::builder::CreateApplicationCommand;

use mongodb::Database;

use crate::db;
use crate::db::data::{
    SleepState,
};
use crate::util::seconds_pretty;

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use futures::stream::TryStreamExt;

use crate::util::{
    interaction,
};

pub const NAME: &str = "stat";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Lets see how well do you sleep...")
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let id = command.user.id.to_string();

    let mut cursor = db::fetch_sleep_all(db, &id).await?;

    let mut sleep_count = 0;
    let mut sleep_total_seconds = 0;

    let mut woke_count = 0;
    let mut woke_total_seconds = 0;

    let mut woke_total = 0;
    let mut sleep_total = 0;

    while let Some(sleep) = cursor.try_next().await? {
        if sleep.time == 0 {
            continue;
        }

        match sleep.state {
            SleepState::Woke => {
                woke_count += 1;
                woke_total_seconds += sleep.time;
                woke_total += sleep.date.timestamp();
            }
            SleepState::Sleep => {
                sleep_count += 1;
                sleep_total_seconds += sleep.time;
                sleep_total += sleep.date.timestamp();
            }
        }
    }

    if woke_count + sleep_count < 3 {
        interaction::send_silent(ctx, command, "You must go through at least one full sleep cycle, before viewing your stats").await;

        return Ok(());
    }

    let woke_average_seconds = woke_total_seconds / woke_count;
    let sleep_average_seconds = sleep_total_seconds / sleep_count;

    let woke_average = woke_total / woke_count;
    let sleep_average = sleep_total / sleep_count;

    let embed = super::colorful_embed()
        .title("Sleep stats")
        .description(format!("Lets see how well <@{}> sleeps...", id))
        .field("Average /woke time", format!("<t:{}:t>", woke_average), false)
        .field("Average /sleep time", format!("<t:{}:t>", sleep_average), false)
        .field("Average awake duration", seconds_pretty(woke_average_seconds), false)
        .field("Average sleep duration", seconds_pretty(sleep_average_seconds), false)
        .clone();

    interaction::send(ctx, command, |message| {
        message
            .set_embed(embed)
    }).await;
    
    Ok(())
}