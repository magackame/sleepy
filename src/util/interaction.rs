use serenity::{prelude::Context, model::prelude::interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType}, builder::{CreateInteractionResponse, CreateInteractionResponseData}};

async fn send_interaction_response<'a, F>(ctx: &Context, command: &ApplicationCommandInteraction, f: F)
where
    for<'b> F:
        FnOnce(&'b mut CreateInteractionResponse<'a>) -> &'b mut CreateInteractionResponse<'a>
{
    let response = command
        .create_interaction_response(&ctx.http, f)
        .await;
    
    if let Err(why) = response {
        println!("Failed to create interaction response: {:?}", why);
    }
}

pub async fn send<'a, F>(ctx: &Context, command: &ApplicationCommandInteraction, f: F)
where
    for<'b> F: FnOnce(&'b mut CreateInteractionResponseData<'a>,) -> &'b mut CreateInteractionResponseData<'a>
{
    send_interaction_response(ctx, command, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(f)
    }).await;
}

pub async fn send_silent(ctx: &Context, command: &ApplicationCommandInteraction, content: &str) {
    send(ctx, command, |message| {
        message
            .ephemeral(true)
            .content(content)
    }).await;
}