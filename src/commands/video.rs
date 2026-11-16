use crate::controllers::video_notification::{VideoNotification, VideoNotificationVariables};
use crate::embeds::video_embed::VideoNotificationType;
use crate::embeds::{self};
use crate::graphql::graphql_query;
use crate::{Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    default_member_permissions = "ADMINISTRATOR",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn video(ctx: Context<'_>, #[description = "Choose id"] id: i32) -> Result<(), Error> {
    ctx.defer().await?;

    let response = graphql_query::<VideoNotification, _>(VideoNotificationVariables { id }).await?;

    let Some(video) = response.and_then(|data| data.video) else {
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
