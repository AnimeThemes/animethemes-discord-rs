use crate::embeds::video_embed::VideoNotificationType;
use crate::embeds::{self};
use crate::graphql::graphql_query;
use crate::graphql::video_notification::{VideoNotification, VideoNotificationVariables};
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn video(ctx: Context<'_>, #[description = "Choose id"] id: i32) -> Result<(), Error> {
    ctx.defer().await?;

    let response = graphql_query::<VideoNotification, _>(VideoNotificationVariables { id }).await?;

    let Some(video) = response.data.and_then(|data| data.video) else {
        ctx.say("Video not found.").await?;
        return Ok(());
    };

    let embed = embeds::video_embed::create_video_notification_embed(
        video.embed,
        VideoNotificationType::Added,
    );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
