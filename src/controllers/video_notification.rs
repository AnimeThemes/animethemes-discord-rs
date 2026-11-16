use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use poise::serenity_prelude as serenity;
use reqwest::StatusCode;
use serde::Deserialize;

use crate::embeds::video_embed::{VideoEmbed, VideoNotificationType};
use crate::graphql::{graphql_query, schema};
use crate::{AppState, embeds};

#[derive(Debug, Deserialize)]
pub struct VideoNotificationBody {
    videos: Vec<Video>,
    r#type: String,
}

impl VideoNotificationBody {
    fn get_type(&self) -> VideoNotificationType {
        match self.r#type.as_str() {
            "added" => VideoNotificationType::Added,
            "updated" => VideoNotificationType::Updated,
            _ => VideoNotificationType::Added,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Video {
    video_id: i32,
}

pub async fn video_notification(
    State(state): State<Arc<AppState>>,
    Json(body): Json<VideoNotificationBody>,
) -> Response {
    let ctx = match state.discord.read().await.clone() {
        Some(ctx) => ctx,
        None => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };

    let channel_id = std::env::var("DISCORD_UPLOADS_CHANNEL_ID")
        .expect("DISCORD_UPLOADS_CHANNEL_ID is required in .env");

    let channel = serenity::ChannelId::new(channel_id.parse::<u64>().unwrap());

    for video in &body.videos {
        let Ok(Some(result)) = graphql_query::<VideoNotification, _>(
            VideoNotificationVariables {
                id: video.video_id,
            },
        )
        .await else {
            continue;
        };

        if let Some(video) = result.video {
            let embed =
                embeds::video_embed::create_video_notification_embed(video.embed, body.get_type());

            let message = serenity::CreateMessage::new().embed(embed);

            if let Err(error) = channel.send_message(&ctx.http, message).await {
                eprintln!("Failed to send video notification: {}", error);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }

    StatusCode::CREATED.into_response()
}

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
