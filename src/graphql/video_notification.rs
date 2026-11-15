use super::schema;
use crate::embeds::video_embed::VideoEmbed;

#[derive(cynic::QueryVariables, Debug)]
pub struct VideoNotificationVariables {
    pub id: i32,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "VideoNotificationVariables")]
pub struct VideoNotification {
    #[arguments(id: $id)]
    pub video: Option<VideoNotificationData>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Video")]
pub struct VideoNotificationData {
    #[cynic(spread)]
    pub embed: VideoEmbed,
}
