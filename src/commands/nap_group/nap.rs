use anyhow::Result;
use serenity::builder::CreateApplicationCommand;

use mongodb::Database;
use serenity::model::prelude::command::CommandOptionType;

use crate::db::data::{
    SleepState,
};

use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub const NAME: &str = "nap";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("You should go to sleep... Now!")
        .create_option(|option| {
            option
                .kind(CommandOptionType::User)
                .name(super::USER_OPTION_NAME)
                .description("That certain somebody that clearly needs some rest")
                .required(true)
        })
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    super::run(SleepState::Sleep, db, ctx, command).await
}