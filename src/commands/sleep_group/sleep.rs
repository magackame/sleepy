use serenity::builder::CreateApplicationCommand;

use mongodb::Database;

use crate::db::data::SleepState;

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub const NAME: &str = "sleep";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Tell everyone that you are going to sleep")
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    super::run(SleepState::Sleep, db, ctx, command).await
}