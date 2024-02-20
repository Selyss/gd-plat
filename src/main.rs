use dotenv::dotenv;
use std::error::Error;
use std::collections::HashMap;

use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, Configuration, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("&")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token = env::var("TOKEN").expect("Error: missing token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error: unable to create client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Error: an error occurred while running the client: {:?}", why);
    }
}

async fn get_data(time: chrono::Utc) -> Result<String, dyn Error> {
    // TODO: need to filter after date uploaded
    let request = HashMap::from([
        ("secret", "Wmfd2893gb7"), // public secret
        ("len", 5), // plat?
        ("diff", -2), // demon
        ("star", 1),
    ]);

    let resp = reqwest::get("http://www.boomlings.com/database/getGJLevels21.php")
        .json(&request)
        .send()
        .await?;

    return resp
}
#[command]
async fn platfind(ctx: &Context, msg: &Message) -> CommandResult {
    let time = chrono::offset::Utc::now();
    let resp = get_data(time).await?;
    msg.reply(ctx, resp).await?;

    Ok(())
}
