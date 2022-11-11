use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::util::interaction;

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    interaction::send_silent(ctx, command, "It seems that application encountered an internal error\nYou can try again later or message the developer at magackame#4728").await;
}