use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::gateway::Activity;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::model::id::ChannelId;

use std::fs::File;
use std::io::prelude::*;
use std::io;

extern crate reqwest;

use serenity::framework::standard::{
    Args,
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use rand::Rng;



extern crate serde_json;
use serde_json::Value as JsonValue;

#[group]
#[commands(test, randint, help, info, cat)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, info: Ready) {
        print!("\x1B[2J\x1B[1;1H");
        println!("Connected with {}", info.user.name);
        let mut file = File::open("config.json").expect("Error opening config file!");

        let mut c = String::new();
        file.read_to_string(&mut c).expect("Error reading file!");

        let res: JsonValue = serde_json::from_str(&c).expect("Error getting Json values");

        _ctx.set_activity(Activity::playing(res["presence"].to_string().replace('"', "").as_str(),)).await;
    }
}






#[tokio::main]
async fn main() {
    let mut file = File::open("config.json").expect("Error opening config file!");

    let mut c = String::new();
    file.read_to_string(&mut c).expect("Error reading file!");

    let res: JsonValue = serde_json::from_str(&c).expect("Error getting Json values");

    let prefix = res["prefix"].to_string().replace('"', "");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&prefix))
        .group(&GENERAL_GROUP);


    let token = res["token"].to_string().replace('"', "");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}


#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(ctx, "This is a test").await;

    Ok(())
}

#[command]
async fn randint(ctx: &Context, msg: &Message) -> CommandResult {
    let num: i64 = rand::thread_rng().gen_range(0..100);

    msg.channel_id.say(&ctx.http, format!("Your number is: {}", num)).await?;
    
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    
    let mut embed = CreateEmbed::default();
    embed.title("RustBot");
    embed.description("All of the commands of the bot");
    embed.field("randint", "Generates a random number from 1 to 100", false);
    embed.footer(|f| {
        f.text(&format!("RustBot by hellsing"))
    });

    msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| {
        e.0 = embed.0;
        e
    })).await?;

    Ok(())
}

#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let mut embed = CreateEmbed::default();
    embed.title("RustBot");
    embed.description("RustBot made by hellsing");
    embed.field("github: ", "[9s6](https://github.com/9s6)", true);
    embed.field("repl.it: ", "[udp](https://repl.it/@udp)", true);
    embed.footer(|f| {
        f.text(&format!("RustBot by hellsing"))
    });
    msg.channel_id.send_message(&ctx.http, |m| m.embed(|e| {
        e.0 = embed.0;
        e
    })).await?;
    
    Ok(())
}


#[command]
async fn cat(ctx: &Context, msg: &Message) -> CommandResult {
    
    let body = reqwest::get("http://aws.random.cat/meow").await?.text().await?;

    let res: JsonValue = serde_json::from_str(&body.as_str()).expect("Error getting Json values");



    msg.channel_id.say(&ctx.http, &res["file"].to_string().replace('"', "")).await?;



    Ok(())
}