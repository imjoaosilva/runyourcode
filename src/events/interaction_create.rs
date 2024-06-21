use super::{commands};
use serenity::all::{Context, Interaction};

pub async fn run(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Command(command) => {
            match command.data.name.as_str() {
                "runcode" => commands::runcode::run(ctx, command).await,
                _ => println!("❌ - Command not found!"),
            }
        }
        _ => println!("🔘 - Some other interaction detected!"),
    }
}
