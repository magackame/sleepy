use serenity::builder::CreateApplicationCommand;

use mongodb::Database;
use serenity::model::prelude::command::CommandOptionType;

use crate::db;
use crate::db::data::UserOptions;

use crate::util::{
    interaction,
    option::ParsedOptions,
};

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub const NAME: &str = "autowoke";
pub const ENABLE_OPTION_NAME: &str = "enable";

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Automatically use /woke when going online after sleep")
        .create_option(|option| {
            option
                .kind(CommandOptionType::Boolean)
                .name(ENABLE_OPTION_NAME)
                .description("True/False for On/Off")
                .required(true)
        })
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let id = command.user.id.to_string();

    let parsed_options = ParsedOptions::parse(&command.data.options);

    let autowoke = parsed_options.get_boolean(ENABLE_OPTION_NAME)?;

    let user = db::fetch_user(db, &id).await?;

    if user.is_none() {
        db::insert_user(db, UserOptions::new(id.clone())).await?;
    }

    db::update_user_autowoke(db, &id, autowoke).await?;

    let content = if autowoke {
        "Successfully enabled 🟢"
    } else {
        "Successfully disabled 🔴"
    };

    interaction::send(ctx, command, |message| {
        message
            .ephemeral(true)
            .content(content)
    }).await;
    
    Ok(())
}