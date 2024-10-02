use dotenv::dotenv;
use serenity::{ all::GatewayIntents, Client };
use std::env;
use z4_runner::{handlers::event::Handler, models::api::ReqwestClientKey};
use env_logger;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("[🛑] Expected a token in the environment");
    
    let debug_mode: u8 = env::var("DEBUG_MODE")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .unwrap_or(0);

    match debug_mode {
        0 => env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Error).init(),
        1 => env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Warn).init(),
        2 => env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Info).init(),
        3 => env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Debug).init(),
        _ => env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Trace).init(),
    }

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
