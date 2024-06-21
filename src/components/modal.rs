use crate::services::api::run_code;
use serenity::all::{
    ActionRow, ActionRowComponent, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, ModalInteraction
};

pub async fn run(ctx: Context, interaction: ModalInteraction) {
    let language = get_text(&interaction.data.components[0]);
    let code = get_text(&interaction.data.components[1]);

    let response = CreateInteractionResponseMessage::new().content("Processing your request...");
    let building_builder = CreateInteractionResponse::Defer(response);

    interaction
        .create_response(&ctx.http, building_builder)
        .await
        .unwrap();

    if let Ok(data) = run_code(language.clone(), code.clone()).await {
        if data.ok {
            let embed = CreateEmbed::default().description(format!(
                "`Input` ```{}\n {}```\n`Output` ```{}\n```",
                language, code, data.stdout
            ));

            let reply_builder = EditInteractionResponse::new().add_embed(embed);

            interaction
                .edit_response(ctx.http, reply_builder)
                .await
                .unwrap();
        } else {
            let embed = CreateEmbed::default().description(format!(
                "`Input:` ```{}\n{}```\n`Error Output`: ```{}\n```",
                language, code, data.stderr
            ));

            let reply_builder = EditInteractionResponse::new().add_embed(embed);

            interaction
                .edit_response(ctx.http, reply_builder)
                .await
                .unwrap();
        }
    } else {
        let reply_builder =
            EditInteractionResponse::new().content(String::from("Failed to execute code!"));

        interaction
            .edit_response(ctx.http, reply_builder)
            .await
            .unwrap();
    }
}

fn get_text(data: &ActionRow) -> String {
    let mut text = String::new();

    for component in &data.components {
        match component {
            ActionRowComponent::InputText(input_text) => {
                text.push_str(&input_text.value.as_ref().unwrap());
            }
            _ => {}
        }
    }

    text
}
