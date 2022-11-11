use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};

use crate::db::data::SleepState;
use crate::util::{
    interaction,
    option::ParsedOptions,
};

use anyhow::Result;

pub mod gm;
pub mod gn;

fn content(ping: Result<bool>, receiver_id: &str) -> String {
    match ping {
        Ok(ping) => {
            if ping {
                format!("<@{}>", receiver_id)
            } else {
                String::new()
            }
        }
        Err(_) => {
            String::new()
        }
    }
}

fn title(state: SleepState) -> &'static str {
    match state {
        SleepState::Woke => "GM!",
        SleepState::Sleep => "GN!",
    }
}

fn description(state: SleepState, application_id: &str, sender_id: &str, receiver_id: &str) -> String {
    if receiver_id == sender_id {
        match state {
            SleepState::Woke => format!("Good morning <@{}> !\nWhat you have planned for today?", sender_id),
            SleepState::Sleep => format!("Good night and sweet dreams <@{}> ~\nI will always be there for you", sender_id),
        }
    } else if receiver_id == application_id {
        match state {
            SleepState::Woke => format!("I wonder how it feels to wake up...\nGood morning <@{}> !", sender_id),
            SleepState::Sleep => format!("Although I never sleep myself\nI wish you a good night's rest <@{}>", sender_id),
        }
    } else {
        match state {
            SleepState::Woke =>  format!("Good morning <@{}> !", receiver_id),
            SleepState::Sleep => format!("Good night and sweet dreams <@{}> ~", receiver_id),
        }
    }
}

const USER_OPTION_NAME: &str = "user";
const PING_OPITON_NAME: &str = "ping";

pub async fn run(state: SleepState, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    let sender_id = command.user.id.to_string();

    let parsed_options = ParsedOptions::parse(&command.data.options);

    let receiver_id = parsed_options.get_user(USER_OPTION_NAME)?.id.to_string();
    let ping = parsed_options.get_boolean(PING_OPITON_NAME);

    let application_id = ctx.cache.current_user_id().to_string();

    let content = content(ping, &receiver_id);
    let title = title(state);
    let description = description(state, &application_id, &sender_id, &receiver_id);

    let mut embed = super::sleep_embed(state);
    
    embed
        .title(title)
        .description(description);

    interaction::send(ctx, command, |message| {
        message
            .content(content)
            .set_embed(embed)
    }).await;

    Ok(())
}