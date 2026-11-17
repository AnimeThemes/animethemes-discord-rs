use indexmap::IndexMap;
use poise::serenity_prelude as serenity;

use crate::descriptions::artist::artists_description;
use crate::descriptions::{self};
use crate::graphql::enums::{ImageFacet, VideoOverlap};
use crate::graphql::schema;
use crate::utils::slugs;

pub enum VideoNotificationType {
    Added,
    Updated,
}

pub fn create_video_notification_embed(
    video: VideoEmbed,
    r#type: VideoNotificationType,
) -> serenity::CreateEmbed {
    let embed = serenity::CreateEmbed::new();

    let mut description: Vec<String> = vec![];

    let entry = video.entries.nodes.first().unwrap();
    let theme = &entry.theme;
    let anime = &theme.anime;

    let theme_slug = slugs::create_theme_slug(&theme.theme_slug, &entry.entry_slug);
    let video_slug =
        slugs::create_video_slug(&theme.theme_slug, &entry.entry_slug, &video.video_slug);
    let video_slug_link = format!("[{}]({}/{})", theme_slug, anime.site_url, video_slug);

    let song_staff = theme
        .song
        .as_ref()
        .map(|song| {
            let mut staff: Vec<_> = song.staff.iter().collect();

            staff.sort_by(|a, b| a.relevance.cmp(&b.relevance));

            staff
        })
        .unwrap_or_default();

    let performers = artists_description(
        song_staff
            .iter()
            .map(|staff| &staff.artist_description)
            .collect(),
        "Performance".to_string(),
    );

    let performers = if !performers.is_empty() {
        format!(" by {}", performers)
    } else {
        "".to_string()
    };

    description.push(match r#type {
        VideoNotificationType::Added => format!("{} has been added.\n", video_slug_link),
        VideoNotificationType::Updated => format!("{} has been updated.\n", video_slug_link),
    });

    description.push(match entry.spoiler {
        true => format!("⚠️ Spoiler"),
        false => "".to_string(),
    });

    description.push(match entry.nsfw {
        true => format!("🔞 NSFW"),
        false => "".to_string(),
    });

    description.push(match &theme.song {
        Some(song) => format!(
            "**Song:** {}{}\n",
            song.title.romaji.as_deref().unwrap_or("*T.B.A.*"),
            performers
        ),
        None => "".to_string(),
    });

    let mut staff_by_role: IndexMap<&str, Vec<_>> = IndexMap::new();

    for staff in &song_staff {
        if staff.role == "Performance" {
            continue;
        }

        staff_by_role
            .entry(staff.role.as_str())
            .or_default()
            .push(&staff.artist_description);
    }

    let staff_descriptions: Vec<String> = staff_by_role
        .into_iter()
        .map(|(role, staff)| format!("{role}: {}", artists_description(staff, role.to_string())))
        .collect();

    if !staff_descriptions.is_empty() {
        description.push(format!("{}\n", staff_descriptions.join("\n")));
    }

    description.push(match &entry.episodes {
        Some(episodes) => format!("**Episodes:** {}", episodes),
        None => "".to_string(),
    });

    description.push(match video.resolution {
        Some(resolution) => format!("**Resolution:** {}p", resolution),
        None => "".to_string(),
    });

    description.push(match video.overlap {
        VideoOverlap::None => "".to_string(),
        _ => format!("**Overlap:** {}", video.overlap_localized),
    });

    description.push(match &entry.notes {
        Some(notes) => format!("\n**Notes:** {}", notes),
        None => "".to_string(),
    });

    let mut embed = embed
        .color(match r#type {
            VideoNotificationType::Added => serenity::Color::from_rgb(46, 204, 113),
            VideoNotificationType::Updated => serenity::Color::from_rgb(255, 255, 0),
        })
        .title(&anime.title.romaji)
        .description(
            description
                .iter()
                .filter(|line| !line.trim().is_empty())
                .cloned()
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .url(&anime.site_url);

    if let Some(image) = anime
        .images
        .nodes
        .iter()
        .find(|i| i.facet == ImageFacet::SmallCover)
        .map(|image| image.link.as_str())
    {
        embed = embed.thumbnail(image);
    }

    if !video.tags.is_empty() {
        embed = embed.footer(serenity::CreateEmbedFooter::new(video.tags));
    }

    embed
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Video")]
pub struct VideoEmbed {
    pub overlap: VideoOverlap,
    pub overlap_localized: String,
    pub resolution: Option<i32>,
    pub source_localized: Option<String>,
    pub tags: String,
    pub entries: VideoEntryConnection,
    #[cynic(spread)]
    pub video_slug: slugs::CreateVideoSlugVideo,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct VideoEntryConnection {
    pub nodes: Vec<Entry>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Entry {
    pub episodes: Option<String>,
    pub notes: Option<String>,
    pub nsfw: bool,
    pub spoiler: bool,
    pub theme: Theme,
    #[cynic(spread)]
    pub entry_slug: slugs::CreateVideoSlugEntry,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Theme {
    pub anime: Anime,
    pub song: Option<Song>,
    #[cynic(spread)]
    pub theme_slug: slugs::CreateVideoSlugTheme,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Song {
    pub title: SongTitle,
    pub staff: Vec<SongStaff>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct SongStaff {
    pub relevance: i32,
    pub role: String,
    #[cynic(spread)]
    pub artist_description: descriptions::artist::ArtistDescriptionFragmentSongStaff,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct SongTitle {
    pub romaji: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Anime {
    pub title: AnimeTitle,
    pub site_url: String,
    pub images: ImageableConnection,
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

#[derive(cynic::QueryFragment, Debug)]
pub struct AnimeTitle {
    pub romaji: String,
}
