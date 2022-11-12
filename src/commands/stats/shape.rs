use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::util::{
    interaction,
};

pub const NAME: &str = "shape";

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let mut embed = super::super::colorful_embed();

    embed
        .title("Shape")
        .description("Shape of stored data")
        .field("state `String`", "Sleep/Woke - state to which you switched\n`/sleep` - puts you in `Sleep` state\n`/woke` - puts you in `Woke` state", false)
        .field("time `Integer`", "Amount of time in seconds for which you were in the given state\nFor example value `3600` with state `Sleep` means that you slept for one hour", false)
        .field("mentions`[String]`", "List of discord user id's that used /nap or /wakey with you as a target", false)
        .field("date `DateTime`", "Date and time in UTC at which state switch occurred", false);
    
    interaction::send(ctx, command, |message| {
        message
            .set_embed(embed)
    }).await;

    Ok(())
}