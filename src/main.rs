mod commands;
use std::collections::HashMap;
use chrono::Utc;
use commands::*;

mod db;
mod util;

mod presence;
use presence::cache::{PresenceCache, status_to_sleep, PresenceCacheEntry};

use db::data::SleepState;
use presence::{presence_handler, start_presence_worker};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{
    Interaction,
};
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Guild, Presence};
use serenity::prelude::*;

struct Handler {
    db: mongodb::Database,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let res = match command.data.name.as_str() {
                sleep_group::sleep::NAME => sleep_group::sleep::run(&self.db, &ctx, &command).await,
                sleep_group::woke::NAME => sleep_group::woke::run(&self.db, &ctx, &command).await,
                gn_group::gn::NAME => gn_group::gn::run(&ctx, &command).await,
                gn_group::gm::NAME => gn_group::gm::run(&ctx, &command).await,
                stat::NAME => stat::run(&self.db, &ctx, &command).await,
                nap_group::nap::NAME => nap_group::nap::run(&self.db, &ctx, &command).await,
                nap_group::wakey::NAME => nap_group::wakey::run(&self.db, &ctx, &command).await,
                sleepyheads_group::sleepyheads::NAME => sleepyheads_group::sleepyheads::run(&self.db, &ctx, &command).await,
                sleepyheads_group::wakers::NAME => sleepyheads_group::wakers::run(&self.db, &ctx, &command).await,
                auto_group::autowoke::NAME => auto_group::autowoke::run(&self.db, &ctx, &command).await,
                auto_group::autosleep::NAME => auto_group::autosleep::run(&self.db, &ctx, &command).await,
                _ => {
                    unknown::run(&ctx, &command).await;

                    Ok(())
                },
            };

            if let Err(why) = res {
                println!("Internal error while handling command: {:?}", why);

                error::run(&ctx, &command).await;
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        guild.set_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                sleepyheads_group::sleepyheads::register(command)
            });

            commands.create_application_command(|command| {
                sleepyheads_group::wakers::register(command)
            })
        })
        .await
        .expect("Faield to set guild application commands");
    }

    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        let mut write_lock = ctx.data.write().await;
        let presence_cache = write_lock.get_mut::<PresenceCache>()
            .expect("Expected PresenceCache in ctx.data TypeMap");

        let presence_state = match status_to_sleep(new_data.status) {
            Ok(presence_state) => presence_state,
            Err(why) => {
                println!("Presence update error: {}", why);
                return;
            }
        };

        match presence_cache.get_mut(&new_data.user.id) {
            Some(entry) => {
                if entry.state != presence_state {
                    entry.state = presence_state;

                    if presence_state == SleepState::Woke {
                        if let Err(why) = presence_handler(&self.db, &ctx, new_data.user.id, presence_state, Utc::now()).await {
                            println!("Presence update error: {}", why);
                        }
                    }
                }
            }
            None => {
                presence_cache.insert(new_data.user.id, PresenceCacheEntry::new(presence_state));
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|command| {
                sleep_group::sleep::register(command)
            });
            
            commands.create_application_command(|command| {
                sleep_group::woke::register(command)
            });

            commands.create_application_command(|command| {
                gn_group::gn::register(command)
            });

            commands.create_application_command(|command| {
                gn_group::gm::register(command)
            });

            commands.create_application_command(|command| {
                stat::register(command)
            });

            commands.create_application_command(|command| {
                nap_group::nap::register(command)
            });

            commands.create_application_command(|command| {
                nap_group::wakey::register(command)
            });

            commands.create_application_command(|command| {
                auto_group::autowoke::register(command)
            });

            commands.create_application_command(|command| {
                auto_group::autosleep::register(command)
            })
        })
        .await
        .expect("Failed to set global application commands");

        let mut write_lock = ctx.data.write().await;
        write_lock.insert::<PresenceCache>(HashMap::new());

        drop(write_lock);

        let db = self.db.clone();

        start_presence_worker(db, ctx);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN")
        .expect("DISCORD_TOKEN env var is not set");

    let mut intents = GatewayIntents::default();

    intents.set(GatewayIntents::GUILDS, true);
    intents.set(GatewayIntents::GUILD_PRESENCES, true);
    intents.set(GatewayIntents::GUILD_MEMBERS, true);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            db: db::connect().await,
        })
        .await
        .expect("Failed to create client");
    
    client.start()
        .await
        .expect("Failed to start client");
}