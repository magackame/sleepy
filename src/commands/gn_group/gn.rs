use serenity::builder::{CreateApplicationCommand};
use serenity::model::prelude::command::CommandOptionType;

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::db::data::SleepState;

pub const NAME: &str = "gn";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Wish that certain somebody a good night's rest")
        .create_option(|option| {
            option
                .kind(CommandOptionType::User)
                .name(super::USER_OPTION_NAME)
                .description("That certain somebody")
                .required(true)
        })
        .create_option(|option| {
            option
                .kind(CommandOptionType::Boolean)
                .name(super::PING_OPITON_NAME)
                .description("Whether to ping user")
                .required(false)
        })
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    super::run(SleepState::Sleep, ctx, command).await
}