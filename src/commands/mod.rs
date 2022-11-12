use chrono::{Duration, Utc, DateTime};
use serenity::builder::CreateEmbed;

use crate::{util::{
    random::{
        color,
        gif,
    },
}, db::data::{SleepState, Sleep}};

pub mod error;
pub mod unknown;
pub mod gn_group;
pub mod stats;
pub mod nap_group;
pub mod sleepyheads_group;
pub mod auto_group;
pub mod sleep_group;

pub fn calc_time(sleep_last: &Sleep, now: DateTime<Utc>) -> i64 {
    let time: Duration = now - sleep_last.date;
    
    time.num_seconds()
}

fn colorful_embed() -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    
    embed.color(color::random());
    
    embed
}

pub fn sleep_embed(state: SleepState) -> CreateEmbed {
    let mut embed = colorful_embed();

    let gif = match state {
        SleepState::Woke => gif::random_gm(),
        SleepState::Sleep => gif::random_gn(),
    };

    embed.image(gif);

    embed
}

fn nap_embed(state: SleepState) -> CreateEmbed {
    let mut embed = colorful_embed();

    let gif = match state {
        SleepState::Woke => gif::random_wakey(),
        SleepState::Sleep => gif::random_nap(),
    };

    embed.image(gif);

    embed
}