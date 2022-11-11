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

pub const NAME: &str = "autosleep";
pub const ENABLE_OPTION_NAME: &str = "enable";
pub const HOURS_OPTION_NAME: &str = "hours";

const MIN_AUTOSLEEP_HOURS: u64 = 2;
const MAX_AUTOSLEEP_HOURS: u64 = 16;
const DEFAULT_AUTOSLEEP_HOURS: u64 = 6;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(NAME)
        .description("Automatically use /sleep when going offline for 6 hours")
        .create_option(|option| {
            option
                .kind(CommandOptionType::Boolean)
                .name(ENABLE_OPTION_NAME)
                .description("True/False for On/Off")
                .required(true)
        })
        .create_option(|option| {
            option
                .kind(CommandOptionType::Integer)
                .name(HOURS_OPTION_NAME)
                .min_int_value(MIN_AUTOSLEEP_HOURS)
                .max_int_value(MAX_AUTOSLEEP_HOURS)
                .description("Delay in hours before using /sleep after offline")
                .required(false)
        })
}

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let id = command.user.id.to_string();

    let parsed_options = ParsedOptions::parse(&command.data.options);

    let enable = parsed_options.get_boolean(ENABLE_OPTION_NAME)?;
    let hours = parsed_options.get_integer(HOURS_OPTION_NAME).unwrap_or(DEFAULT_AUTOSLEEP_HOURS as i64);

    if hours < MIN_AUTOSLEEP_HOURS as i64 || hours > MAX_AUTOSLEEP_HOURS as i64 {
        let content = format!("`hours` should be in range [{}; {}]", MIN_AUTOSLEEP_HOURS, MAX_AUTOSLEEP_HOURS);

        interaction::send_silent(ctx, command, &content).await;

        return Ok(());
    }

    let user = db::fetch_user(db, &id).await?;

    if user.is_none() {
        db::insert_user(db, UserOptions::new(id.clone())).await?;
    }

    let autosleep = if enable {
        Some(hours)
    } else {
        None
    };

    db::update_user_autosleep(db, &id, autosleep).await?;

    let content = if enable {
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