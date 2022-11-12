use std::borrow::Cow;

use mongodb::Database;
use serenity::model::prelude::AttachmentType;

use crate::db;
use crate::db::data::DownloadSleep;

use anyhow::Result;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use futures::stream::TryStreamExt;

use crate::util::{
    interaction,
};

pub const NAME: &str = "download";

pub async fn run(db: &Database, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<()> {
    interaction::send_deffered(ctx, command, |message| {
        message
            .content("Creating JSON files...")
    }).await;

    let id = command.user.id.to_string();
    let mut cursor = db::fetch_sleep_all(db, &id).await?;

    let mut buffer = Vec::new();
    while let Some(sleep) = cursor.try_next().await? {
        buffer.push(DownloadSleep::from(sleep));
    }

    if buffer.is_empty() {
        interaction::send_followup(ctx, command, |message| {
            message
                .content("Your stats are empty!")
        }).await;

        return Ok(());
    }

    let json = serde_json::to_string_pretty(&buffer)?;

    interaction::send_followup(ctx, command, |message| {
        message
            .add_file(AttachmentType::Bytes {
                data: Cow::Borrowed(json.as_bytes()),
                filename: "stats.json".to_string()
            })
    }).await;

    Ok(())
}