use crate::graphql::schema;

pub fn create_theme_slug(theme: &CreateVideoSlugTheme, entry: &CreateVideoSlugEntry) -> String {
    let sequence = match theme.sequence {
        Some(sequence) => sequence,
        None => 1,
    };

    let slug = format!("{}{}", theme.type_localized, sequence);

    let slug = if entry.version != 1 {
        format!("{}v{}", slug, entry.version)
    } else {
        slug
    };

    let slug = if let Some(group) = &theme.group {
        format!("{}-{}", slug, group.slug)
    } else {
        slug
    };

    slug
}

pub fn create_video_slug(
    theme: &CreateVideoSlugTheme,
    entry: &CreateVideoSlugEntry,
    video: &CreateVideoSlugVideo,
) -> String {
    let slug = create_theme_slug(theme, entry);

    let slug = if !video.tags.is_empty() {
        format!("{}-{}", slug, video.tags)
    } else {
        slug
    };

    slug
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Theme")]
pub struct CreateVideoSlugTheme {
    pub type_localized: String,
    pub sequence: Option<i32>,
    pub group: Option<ThemeGroup>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Entry")]
pub struct CreateVideoSlugEntry {
    pub version: i32,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ThemeGroup {
    pub slug: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Video")]
pub struct CreateVideoSlugVideo {
    pub tags: String,
}
