use serenity::all::{
    CommandInteraction, Context, CreateActionRow, CreateCommand, CreateInputText,
    CreateInteractionResponse, CreateModal, InputTextStyle, InteractionContext,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let code_input_text = CreateInputText::new(InputTextStyle::Paragraph, "code", "code");

    let modal_code_component = CreateActionRow::InputText(code_input_text);

    let modal = CreateModal::new("expand_modal", "Expand your Macro")
        .components(vec![modal_code_component]);

    let response = CreateInteractionResponse::Modal(modal);
    command.create_response(&ctx.http, response).await.unwrap()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("rustexpand")
        .description("Expand rust macros.")
        .add_integration_type(serenity::all::InstallationContext::User)
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::BotDm,
            InteractionContext::PrivateChannel,
        ])
}   