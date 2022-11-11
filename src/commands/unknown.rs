use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::util::interaction;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    interaction::send_silent(ctx, command, "Unknown command").await
}