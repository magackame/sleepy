use mongodb::Database;

use crate::db;
use crate::util::{
    option::ParsedOptions,
};

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction, CommandDataOption};
use serenity::prelude::Context;

use crate::util::{
    interaction,
};

pub const NAME: &str = "delete";
pub const CONFIRM_OPTION_NAME: &str = "confirm";
pub const CONFIRMATION_STRING: &str = "sleepy";

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction, sub_command: &CommandDataOption) -> Result<()> {
    let id = command.user.id.to_string();
    let parsed_options = ParsedOptions::parse(&sub_command.options);

    let confirm = parsed_options.get_string(CONFIRM_OPTION_NAME)?;

    let mut embed = crate::commands::colorful_embed();

    interaction::send_deffered(ctx, command, |message| {
        message
            .content("Deleting...")
    }).await;

    if confirm == CONFIRMATION_STRING {
        db::delete_sleep_all(db, &id).await?;

        embed
            .title("DELETED")
            .description("Stats successfully deleted")
            .image("https://media.tenor.com/EGreuR_aoP0AAAAM/goodbye-im-out.gif");
    } else {
        embed
            .title("KEPT")
            .description("Invalid confirmation string\nStats kept")
            .image("https://media.tenor.com/1XP131qTLZoAAAAM/save-anime.gif");
    }

    interaction::send_followup(ctx, command, |message| {
        message
            .set_embed(embed)
    }).await;

    Ok(())
}