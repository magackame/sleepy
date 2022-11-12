use serenity::{prelude::Context, model::prelude::interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType}, builder::{CreateInteractionResponse, CreateInteractionResponseData, CreateInteractionResponseFollowup}};

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

async fn send_interaction_followup<'a, F>(ctx: &Context, command: &ApplicationCommandInteraction, f: F)
where
    for<'b> F:
        FnOnce(&'b mut CreateInteractionResponseFollowup<'a>) -> &'b mut CreateInteractionResponseFollowup<'a>
{
    let response = command
        .create_followup_message(&ctx.http, f)
        .await;
    
    if let Err(why) = response {
        println!("Failed to create interaction response: {:?}", why);
    }
}

async fn send_kind<'a, F>(kind: InteractionResponseType, ctx: &Context, command: &ApplicationCommandInteraction, f: F)
where
    for<'b> F: FnOnce(&'b mut CreateInteractionResponseData<'a>,) -> &'b mut CreateInteractionResponseData<'a>
{
    send_interaction_response(ctx, command, |response| {
        response
            .kind(kind)
            .interaction_response_data(f)
    }).await;
}

pub async fn send<'a, F>(ctx: &Context, command: &ApplicationCommandInteraction, f: F)
where
    for<'b> F: FnOnce(&'b mut CreateInteractionResponseData<'a>,) -> &'b mut CreateInteractionResponseData<'a>
{
    send_kind(InteractionResponseType::ChannelMessageWithSource, ctx, command, f).await;
}

pub async fn send_deffered<'a, F>(ctx: &Context, command: &ApplicationCommandInteraction, f: F)
where
    for<'b> F: FnOnce(&'b mut CreateInteractionResponseData<'a>,) -> &'b mut CreateInteractionResponseData<'a>
{
    send_kind(InteractionResponseType::DeferredChannelMessageWithSource, ctx, command, f).await;
}

pub async fn send_followup<'a, F>(ctx: &Context, command: &ApplicationCommandInteraction, f: F)
where
    for<'b> F:
        FnOnce(&'b mut CreateInteractionResponseFollowup<'a>) -> &'b mut CreateInteractionResponseFollowup<'a>
{
    send_interaction_followup(ctx, command, f).await;
}

pub async fn send_silent(ctx: &Context, command: &ApplicationCommandInteraction, content: &str) {
    send(ctx, command, |message| {
        message
            .ephemeral(true)
            .content(content)
    }).await;
}