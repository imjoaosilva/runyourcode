use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateCommandOption, CreateEmbed, EditInteractionResponse, InteractionContext
};
use super::super::services::api::run_code;

pub async fn run(ctx: Context, command: CommandInteraction) {
    
    let language = command.data.options.get(0).unwrap().value.as_str().unwrap().to_string();
    let code = command.data.options.get(1).unwrap().value.as_str().unwrap().to_string();
    command.defer(&ctx.http).await.unwrap();

    if let Ok(data) = run_code(language.clone(), code.clone()).await {
        
        if data.ok {
            let embed = CreateEmbed::default()
                .description(format!("`Input` ```{}\n {}```\n`Output` ```{}\n```", language, code, data.stdout));
            
            let reply_builder = EditInteractionResponse::new()
                .add_embed(embed);
            
            command.edit_response(ctx.http, reply_builder).await.unwrap();
        } else {
            let embed = CreateEmbed::default()
                .description(format!("Input: ```{}\n{}```\nError Output: ```{}\n```", language, code, data.stderr));
            
            let reply_builder = EditInteractionResponse::new()
                .add_embed(embed);
            
            command.edit_response(ctx.http, reply_builder).await.unwrap();
        }
    } else {

        let reply_builder = EditInteractionResponse::new()
            .content(String::from("Failed to execute code!"));

        command.edit_response(ctx.http, reply_builder).await.unwrap();
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("runcode")
        .description("Execute code.")
        .add_integration_type(serenity::all::InstallationContext::User)
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::BotDm,
            InteractionContext::PrivateChannel,
        ])
        .add_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "language", "The language of the code.").required(true))
        .add_option(CreateCommandOption::new(serenity::all::CommandOptionType::String, "code", "The code to execute.").required(true))
}
