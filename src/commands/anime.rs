use std::sync::LazyLock;
use std::time::Duration;

use poise::serenity_prelude::futures::StreamExt;
use poise::serenity_prelude::{
    self as serenity, ComponentInteractionCollector, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::graphql::enums::ImageFacet;
use crate::graphql::{graphql_query, schema};
use crate::{Context, Error};

/// Searches for an anime by its title or alternative titles.
#[poise::command(slash_command, guild_only)]
pub async fn anime(
    ctx: Context<'_>,
    #[description = "The anime title to search for"] title: String,
) -> Result<(), Error> {
    tracing::info!("[Request from {}] [anime]: {title}", ctx.author().id);

    ctx.defer().await?;

    let Some(data) =
        graphql_query::<SearchQuery, _>(SearchQueryVariables { search: title }).await?
    else {
        ctx.say("Some error occurred.").await?;
        return Ok(());
    };

    let animes = data.search.anime.data;
    let total = animes.len();

    if total == 0 {
        ctx.say("No anime found.").await?;
        return Ok(());
    }

    let mut index = 0;

    let reply = ctx
        .send(
            poise::CreateReply::default()
                .embed(create_embed(&animes[index], index, total))
                .components(navigation_buttons(false)),
        )
        .await?;

    let message_id = reply.message().await?.id;

    let collector = ComponentInteractionCollector::new(ctx.serenity_context().shard.clone())
        .message_id(message_id)
        .timeout(Duration::from_secs(60))
        .stream();

    tokio::pin!(collector);

    while let Some(interaction) = collector.next().await {
        if interaction.user.id != ctx.author().id {
            interaction
                .create_response(
                    ctx.serenity_context(),
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("You are not the one who initiated this command."),
                    ),
                )
                .await?;

            continue;
        }

        index = match interaction.data.custom_id.as_str() {
            "veryPrevious" => 0,
            "veryNext" => total - 1,
            "previous" => (index + total - 1) % total,
            "next" => (index + 1) % total,
            _ => {
                interaction
                    .create_response(
                        ctx.serenity_context(),
                        CreateInteractionResponse::Acknowledge,
                    )
                    .await?;

                continue;
            }
        };

        interaction
            .create_response(
                ctx.serenity_context(),
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(create_embed(&animes[index], index, total))
                        .components(navigation_buttons(false)),
                ),
            )
            .await?;
    }

    reply
        .edit(
            ctx,
            poise::CreateReply::default()
                .embed(create_embed(&animes[index], index, total))
                .components(navigation_buttons(true)),
        )
        .await?;

    Ok(())
}

fn create_embed(anime: &Anime, index: usize, count: usize) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::new()
        .title(anime.title.romaji.clone())
        .url(anime.site_url.clone())
        .color(serenity::model::Colour::BLUE);

    static MULTIPLE_NEWLINES: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"\n{3,}").unwrap());

    if let Some(synopsis) = anime.synopsis.clone() {
        let text = synopsis.replace("\r\n", "\n");

        let text = MULTIPLE_NEWLINES.replace_all(&text, "\n\n");

        embed = embed.description(text.trim().to_string());
    }

    if let Some(image) = anime
        .images
        .nodes
        .iter()
        .find(|i| i.facet == ImageFacet::SmallCover)
        .map(|image| image.link.as_str())
    {
        embed = embed.thumbnail(image);
    }

    let footer = if let Some(format) = anime.format_localized.clone()
        && let Some(season) = anime.season_localized.clone()
        && let Some(year) = anime.year
    {
        format!("{} • {} {} • {}/{}", format, season, year, index + 1, count)
    } else {
        format!("{}/{}", index + 1, count)
    };

    embed.footer(serenity::CreateEmbedFooter::new(footer))
}

fn navigation_buttons(disabled: bool) -> Vec<serenity::CreateActionRow> {
    vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("veryPrevious")
            .emoji(serenity::ReactionType::Unicode("⏪".into()))
            .style(serenity::ButtonStyle::Primary)
            .disabled(disabled),
        serenity::CreateButton::new("previous")
            .emoji(serenity::ReactionType::Unicode("◀️".into()))
            .style(serenity::ButtonStyle::Primary)
            .disabled(disabled),
        serenity::CreateButton::new("next")
            .emoji(serenity::ReactionType::Unicode("▶️".into()))
            .style(serenity::ButtonStyle::Primary)
            .disabled(disabled),
        serenity::CreateButton::new("veryNext")
            .emoji(serenity::ReactionType::Unicode("⏩".into()))
            .style(serenity::ButtonStyle::Primary)
            .disabled(disabled),
    ])]
}

#[derive(cynic::QueryVariables, Debug)]
pub struct SearchQueryVariables {
    pub search: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "SearchQueryVariables")]
pub struct SearchQuery {
    #[arguments(search: $search)]
    pub search: Search,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Search {
    pub anime: AnimePagination,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnimePagination {
    pub data: Vec<Anime>,
    pub page_info: OffsetPageInfo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Anime {
    pub format_localized: Option<String>,
    pub title: AnimeTitle,
    pub site_url: String,
    pub season_localized: Option<String>,
    pub synopsis: Option<String>,
    pub year: Option<i32>,
    pub images: ImageableConnection,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct OffsetPageInfo {
    pub has_next_page: bool,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnimeTitle {
    pub romaji: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ImageableConnection {
    pub nodes: Vec<Image>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Image {
    pub facet: ImageFacet,
    pub link: String,
}
