use serenity::all::CreateCommand;
use super::commands;

pub fn register_commands() -> Vec<CreateCommand> {
    vec![
        commands::runcode::register(),
        commands::expand::register(),
        commands::dontasktoask::register(),
    ]
}