use serenity::all::Context;
use super::handlers::command::register_commands;
use serenity::model::application::Command;

pub async fn run(ctx: Context, ready: serenity::all::Ready) {
    
    match Command::set_global_commands(&ctx.http, register_commands()).await {
        Ok(list) => println!("☑️  - {} Global  Commands loaded!", list.len()),
        Err(_) => println!("❌ - Unable to load commands!"),
    };
    
    println!("✅ - {} started successfully!", ready.user.name);
}
