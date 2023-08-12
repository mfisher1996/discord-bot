use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::settings::Settings;


/// Handler for any message sent in the server
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.to_lowercase().as_str() {
            "ping" => {
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

pub async fn get_client() -> Result<Client, SerenityError> {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        // this is Where the general struct that was created by the group macro comes into play
        .group(&crate::commands::text_commands::GENERAL_GROUP);
    Client::builder(Settings::new().bot.token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
}
