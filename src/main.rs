use dotenv::dotenv;
use reqwest::Error;

use std::env;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, Configuration, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(platfind)]
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
        println!(
            "Error: an error occurred while running the client: {:?}",
            why
        );
    }
}

async fn get_data(time: chrono::DateTime<chrono::Utc>) -> Result<reqwest::Response, Error> {
    // TODO: need to filter after date uploaded
    let request = [
        ("secret", "Wmfd2893gb7"), // public secret
        ("len", "5"),              // plat?
        ("diff", "-2"),            // demon
        ("star", "1"),
    ];

    let client = reqwest::Client::new();
    let resp = client
        .post("http://www.boomlings.com/database/getGJLevels21.php")
        .query(&request)
        .header("User-Agent", "")
        .send()
        .await?;

    Ok(resp)
}
#[command]
async fn platfind(ctx: &Context, msg: &Message) -> CommandResult {
    let time = chrono::offset::Utc::now();
    let resp = match get_data(time).await {
        Ok(resp) => resp,
        Err(e) => {
            msg.reply(ctx, format!("Error: {}", e)).await?;
            return Ok(());
        }
    };

    let body = resp.text().await?;
    msg.reply(ctx, body).await?;

    Ok(())
}
