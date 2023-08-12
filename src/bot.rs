use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::framework::StandardFramework;
use serenity::futures::StreamExt;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::settings::Settings;

// Structs for chanel groups
#[group]
#[commands(ping, homework)]
struct General;

#[group]
#[commands(ping, homework)]
struct Simpcinati;

#[group]
#[commands(ping, homework)]
struct Bragging;

#[group("pc-building")]
#[commands(ping, homework)]
struct PcBuilding;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.to_lowercase().as_str() {
            "fat balls in my mouth" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    log::error!("Error sending response: {}", why);
                }
            }
            _ => (),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("{} is connected", ready.user.name);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
async fn homework(ctx: &Context, msg: &Message) -> CommandResult {
    let channel = msg.channel_id;
    channel.say(&ctx.http, "What would you like my help with?").await?;

    let mut msgs = Box::pin(channel.messages_iter(&ctx.http));
    match msgs.next().await {
        Some(Ok(m)) => {
            let content = m.content;
            match content.parse::<i32>() {
                Ok(num) => {
                    msg.reply(&ctx.http, num).await?;
                }
                Err(_) => {
                    channel.say(&ctx.http, content).await?;
                }
            }
        }
        Some(Err(why)) => {
            log::error!("Error receiving message: {:?}", why);
        }
        None => return Ok(()),    
    }
    Ok(())
}

pub async fn get_client() -> Result<Client, SerenityError> {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    Client::builder(Settings::new().bot.token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
}
