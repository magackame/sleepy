use serenity::builder::CreateApplicationCommand;

use mongodb::Database;
use serenity::model::prelude::command::CommandOptionType;

use crate::util::{
    option::ParsedOptions,
};

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::util::{
    interaction,
};

mod view;
mod shape;
mod delete;
mod download;

pub const NAME: &str = "stats";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Sleep stats commands")
        .create_option(|option| {
            option
                .kind(CommandOptionType::SubCommand)
                .name(view::NAME)
                .description("View your stats")
        })
        .create_option(|option| {
            option
                .kind(CommandOptionType::SubCommand)
                .name(shape::NAME)
                .description("Shape of stored data")
        })
        .create_option(|option| {
            option
                .kind(CommandOptionType::SubCommand)
                .name(delete::NAME)
                .description("Deletes all stats. Operation is irreversible")
                .create_sub_option(|sub_option| {
                    sub_option
                        .kind(CommandOptionType::String)
                        .name(delete::CONFIRM_OPTION_NAME)
                        .description(format!("Type in '{}' to confirm deletion", delete::CONFIRMATION_STRING))
                        .required(true)
                })
        })
        .create_option(|option| {
            option
                .kind(CommandOptionType::SubCommand)
                .name(download::NAME)
                .description("Downloads your stats as JSON file")
        })
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let parsed_options = ParsedOptions::parse(&command.data.options);

    let sub_command = parsed_options.get_sub_command()?;

    match sub_command.name.as_str() {
        view::NAME => view::run(db, ctx, command).await,
        shape::NAME => shape::run(ctx, command).await,
        delete::NAME => delete::run(db, ctx, command, sub_command).await,
        download::NAME => download::run(db, ctx, command).await,
        _ => {
            interaction::send_silent(ctx, command, "Unknown sub command").await;

            Ok(())
        },
    }
}