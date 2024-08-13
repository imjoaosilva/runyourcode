use dotenv::dotenv;
use serenity::{all::GatewayIntents, Client};
use std::env;
use z4_runner::handlers::event::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("[🛑] Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("[🛑] - Error creating client!");

    if let Err(err) = client.start().await {
        println!("[🛑] - Client error: {err:?}");
    }
}
