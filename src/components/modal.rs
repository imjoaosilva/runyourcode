use crate::services::api::run_code;
use serenity::all::{
    ActionRow, ActionRowComponent, Context, CreateEmbed, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
    ModalInteraction,
};
use std::time::Instant;

pub async fn run(ctx: Context, interaction: ModalInteraction) {
    let language = get_text(&interaction.data.components[0]);
    let code = get_text(&interaction.data.components[1]);
    let start = Instant::now();

    let response = CreateInteractionResponseMessage::new().content("Processing your request...");
    let building_builder = CreateInteractionResponse::Defer(response);

    interaction
        .create_response(&ctx.http, building_builder)
        .await
        .unwrap();

    if let Ok(data) = run_code(language.clone(), code.clone()).await {
        if data.ok {
            let footer = CreateEmbedFooter::new(format!("Language: {}", language));

            if data.stderr != String::new() {
                let duration = start.elapsed();
                let embed = CreateEmbed::default()
                    .title(format!("Execution Time: {:?}", duration))
                    .footer(footer)
                    .description(format!(
                        "`Input:` ```{}\n{}```\n`Error Output`: ```{}\n{}```",
                        language, code, language, data.stderr
                    ));

                let reply_builder = EditInteractionResponse::new().add_embed(embed);

                interaction
                    .edit_response(ctx.http, reply_builder)
                    .await
                    .unwrap();
                return;
            }

            let desc = format!(
                "`Input` ```{}\n{}```\n`Output` ```{}\n{}```",
                language, code, language, data.stdout
            );

            let embed = CreateEmbed::default()
                .title(format!("Execution Time: {:?}", start.elapsed()))
                .footer(footer)
                .description(&desc);

            let reply_builder = EditInteractionResponse::new().add_embed(embed);

            if desc.len() > 4095 {
                let reply_builder =
                    EditInteractionResponse::new().content("Output too long to display!");

                interaction
                    .edit_response(ctx.http, reply_builder)
                    .await
                    .unwrap();
                return;
            }

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
