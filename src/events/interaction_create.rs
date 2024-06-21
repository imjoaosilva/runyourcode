use crate::components;

use super::commands;
use serenity::all::{Context, Interaction};

pub async fn run(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Command(command) => match command.data.name.as_str() {
            "runcode" => commands::runcode::run(ctx, command).await,
            _ => println!("❌ - Command not found!"),
        },
        Interaction::Modal(component) => match component.data.custom_id.as_str() {
            "runcode_modal" => components::modal::run(ctx, component).await,
            _ => println!("❌ - Modal not found!"),
        },
        _ => println!("🔘 - Some other interaction detected!"),
    }
}
