use serenity::builder::{CreateApplicationCommand};

use mongodb::Database;
use serenity::model::prelude::command::CommandOptionType;

use crate::db::data::{
    SleepState,
};

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub const NAME: &str = "wakey";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Wakey~wakey~ its time for school!")
        .create_option(|option| {
            option
                .kind(CommandOptionType::User)
                .name(super::USER_OPTION_NAME)
                .description("That certain somebody that should already get up and going")
                .required(true)
        })
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    super::run(SleepState::Woke, db, ctx, command).await
}