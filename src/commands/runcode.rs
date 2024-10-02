use serenity::all::{
    CommandInteraction, Context, CreateActionRow, CreateCommand, CreateInputText,
    CreateInteractionResponse, CreateModal, InputTextStyle, InteractionContext,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let language_input_text = CreateInputText::new(InputTextStyle::Short, "language", "language")
        .placeholder("javascript, python, rust etc");
    let code_input_text = CreateInputText::new(InputTextStyle::Paragraph, "code", "code");

    let modal_language_component = CreateActionRow::InputText(language_input_text);
    let modal_code_component = CreateActionRow::InputText(code_input_text);

    let modal = CreateModal::new("runcode_modal", "RunYourCode")
        .components(vec![modal_language_component, modal_code_component]);

    let response = CreateInteractionResponse::Modal(modal);
    command.create_response(&ctx.http, response).await.unwrap();

    println!("User: {}", command.user.name)
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
}
