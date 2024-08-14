use crate::{models::api::ReqwestClientKey, services::api::rust_expand};
use serenity::all::{
    ActionRow, ActionRowComponent, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, ModalInteraction
};
use std::time::Instant;

pub async fn run(ctx: Context, interaction: ModalInteraction) {
    let data = ctx.data.read().await;
    let api_client = data.get::<ReqwestClientKey>().unwrap();
    
    let code = get_text(&interaction.data.components[0]);
    let start = Instant::now();

    let response = CreateInteractionResponseMessage::new().content("Processing your request...");
    let building_builder = CreateInteractionResponse::Defer(response);

    interaction
        .create_response(&ctx.http, building_builder)
        .await
        .unwrap();

    if let Ok(data) = rust_expand(String::from("2021"), code.clone(), api_client).await {
        if data.success {
            let embed = CreateEmbed::default()
                .title(format!("Execution Time: {:?}", start.elapsed()))
                .description(format!(
                "`Input` ```{}\n{}```\n`Output` ```{}\n{}```",
                "rust", code, "rust", data.stdout
            ));

            let reply_builder = EditInteractionResponse::new().add_embed(embed);

            interaction
                .edit_response(ctx.http, reply_builder)
                .await
                .unwrap();
        } else {
            let embed = CreateEmbed::default()
                .title(format!("Execution Time: {:?}", start.elapsed()))
                .description(format!(
                "`Input:` ```{}\n{}```\n`Error Output`: ```{}\n{}```",
                "rust", code, "rust", data.stderr
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