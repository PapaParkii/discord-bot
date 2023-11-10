/*
Parker Jensen
11/08/23
This Rust program create the functionality for a Discord Bot 
*/

use serenity::{ //using serenity - a rust library for the Discord API
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello there, Human!

? Need help?
=> Post in the <#1172368684082270270> channel and other humans will assist you.

-- BotMan
"; //help message

const HELLO_MESSAGE: &str = "Hello there, fleshy Human!"; //hello message

const COMPLIMENT_MESSAGE: &str = "You gem. You absolute masterpiece of God. You shining piece of gold. You are a piece of art,that The Angels drawn of Earth,and forgot the paint brush. You have a freckle on your neck. Did you know that?
It´s rather cute."; //compliment message

const HELP_COMMAND: &str = "!help"; //Command user should input for the associated 'help' command
const HELLO_COMMAND: &str = "!hello"; //Command user should input for the associated 'hello' command
const COMPLIMENT_COMMAND: &str = "!compliment"; //Command user should input for the associated 'compliment' command

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND { //if the user inputs the help command
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await { //say the help message
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content == HELLO_COMMAND { //if the user inputs the hello command
            if let Err(why) = msg.channel_id.say(&ctx.http, HELLO_MESSAGE).await { //say the hello message
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content == COMPLIMENT_COMMAND { //if the user inputs the compliment command
            if let Err(why) = msg.channel_id.say(&ctx.http, COMPLIMENT_MESSAGE).await { //say the compliment message
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name); //message in the terminal letting me know that the bot has been connected to the server
    }
}

#[tokio::main]
async fn main() {
    let token = "MTE3MjM4ODYyNDY4ODQ4MDI1Ng.GSevCW.HgGr9McO-hG-YTwkHwDURJbfrZuee5pD6DE59Q"; //unique bot token
    let application_id = 1172388624688480256;  //app id associated within the Discord development tools

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await //building the bot using Tokio - it connects the token through the application id and connects to the associated server(s)
        .expect("Error creating client"); //error

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why); //print debug information 
    }
}