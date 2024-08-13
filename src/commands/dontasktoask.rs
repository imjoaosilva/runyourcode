use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, CreateCommand, InteractionContext,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let embed = CreateEmbed::new()
        .title("Don't ask to ask")
        .description("Imagine if everyone asked if they could ask a question before actually asking it. \n
            **For example:** \n
            *User1:* \"Can I ask you something?\" \n
            *User2:* \"Any rust devs here?\" \n
            It would be chaos! Just ask your question directly. \n\n
            **The developers thank you** <:dev:1272932870238896138>")
        .color(0x87CEEB);

    let message = CreateInteractionResponseMessage::new().add_embed(embed);

    command.create_response(&ctx.http, CreateInteractionResponse::Message(message)).await.unwrap();

}

pub fn register() -> CreateCommand {
    CreateCommand::new("dontasktoask")
        .description("Ask to ask? Kek.")
        .add_integration_type(serenity::all::InstallationContext::User)
        .contexts(vec![
            InteractionContext::Guild,
            InteractionContext::BotDm,
            InteractionContext::PrivateChannel,
        ])
}