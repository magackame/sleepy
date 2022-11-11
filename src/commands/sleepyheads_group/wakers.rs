use serenity::builder::CreateApplicationCommand;

use mongodb::Database;

use crate::db::{data::SleepState};

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub const NAME: &str = "wakers";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("See who's wide-awake")
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    super::run(SleepState::Woke, db, ctx, command).await
}