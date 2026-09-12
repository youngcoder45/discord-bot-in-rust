
// Importing all essentials
// Like std::env for getting env vars like DISCORD TOKEN and DATABASE URL.
// And Serenity Which is a package to handle discord bots here
use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler; // Defined a Handler to handler the commands ran by user

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx:Context, msg:Message){
        if msg.content=="!ping"{ // basically bot looks for a message with string "!ping" across the server and Says the string "Pong" snet via http if the command ran was found or returns  the error if any failure is detected.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await{
                println!("Error While Sending the message : {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main(){
    dotenvy::dotenv().ok();
    let token  = env::var("DISCORD_TOKEN").expect("Expected a Discord Token in The environment .env");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token,intents).event_handler(Handler).await.expect("Error While Creating Client");
    if let Err(why) = client.start().await{
        println!("Client error {why:?}");
    }

}
