use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::futures::StreamExt;

/// Command framework for the bot
#[group]
#[commands(ping, homework)]
struct General;

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

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

