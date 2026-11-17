use std::time::Duration;

use poise::serenity_prelude as serenity;
use tokio::time::sleep;

use crate::Error;
use crate::graphql::{graphql_query, schema};
use crate::utils::slugs::{
    CreateVideoSlugEntry, CreateVideoSlugTheme, CreateVideoSlugVideo, create_video_slug,
};

pub async fn ready(ctx: &serenity::Context) -> Result<(), Error> {
    tracing::info!("Bot Online");

    loop {
        let response = graphql_query::<CurrentFeaturedThemeQuery, _>(()).await?;

        if let Some(featured_theme) = response.unwrap_or_default().current_featured_theme {
            let entry = &featured_theme.entry;
            let theme = &entry.theme;

            let name = format!(
                "{} {}",
                entry.theme.anime.title.romaji,
                create_video_slug(
                    &theme.video_slug_theme,
                    &entry.video_slug_entry,
                    &featured_theme.video.video_slug_video
                )
            );

            ctx.set_activity(Some(serenity::ActivityData::listening(name.clone())));

            tracing::info!("[ready] Set featured theme to {name}");
        }

        sleep(Duration::from_mins(30)).await;
    }
}

#[derive(cynic::QueryFragment, Debug, Default)]
#[cynic(graphql_type = "Query")]
pub struct CurrentFeaturedThemeQuery {
    pub current_featured_theme: Option<CurrentFeaturedTheme>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct CurrentFeaturedTheme {
    pub entry: Entry,
    pub video: Video,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Entry {
    #[cynic(spread)]
    pub video_slug_entry: CreateVideoSlugEntry,
    pub theme: Theme,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Theme {
    #[cynic(spread)]
    pub video_slug_theme: CreateVideoSlugTheme,
    pub anime: Anime,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Anime {
    pub title: AnimeTitle,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct AnimeTitle {
    pub romaji: String,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Video {
    #[cynic(spread)]
    pub video_slug_video: CreateVideoSlugVideo,
}
