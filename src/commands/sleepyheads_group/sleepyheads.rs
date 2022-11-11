use serenity::builder::CreateApplicationCommand;
use mongodb::Database;

use crate::db::{data::SleepState};

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub const NAME: &str = "sleepyheads";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("See who's a sleepyhead")
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    super::run(SleepState::Sleep, db, ctx, command).await
}