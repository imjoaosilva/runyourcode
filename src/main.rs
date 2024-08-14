use dotenv::dotenv;
use serenity::{ all::GatewayIntents, Client };
use std::env;
use z4_runner::{handlers::event::Handler, models::api::ReqwestClientKey};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("[🛑] Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler).await
        .expect("[🛑] - Error creating client!");

    let api_client = reqwest::Client::new();

    {
        let mut data = client.data.write().await;
        data.insert::<ReqwestClientKey>(api_client);
    }

    if let Err(err) = client.start().await {
        println!("[🛑] - Client error: {err:?}");
    }
}
