use poise::{Modal as _, serenity_prelude as serenity};

use crate::{Data, Error};

type Context<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(Debug, poise::Modal)]
#[name = "Message"]
struct SayModal {
    #[name = "Message"]
    #[paragraph]
    #[max_length = 2000]
    message: String,

    #[name = "Image Link"]
    #[paragraph]
    #[placeholder = "Comma-separated Links"]
    image_links: Option<String>,
}

#[poise::command(
    slash_command,
    guild_only,
    ephemeral,
    default_member_permissions = "ADMINISTRATOR",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Set a channel"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let Some(input) = SayModal::execute(ctx).await? else {
        return Ok(());
    };

    let mut files = Vec::new();

    if let Some(links) = input.image_links {
        let urls: Vec<&str> = links
            .split(',')
            .map(str::trim)
            .filter(|url| !url.is_empty())
            .collect();

        if urls.len() > 10 {
            ctx.say("Maximum of 10 attachments allowed.").await?;
            return Ok(());
        }

        for url in urls {
            match serenity::CreateAttachment::url(ctx.http(), url).await {
                Ok(attachment) => files.push(attachment),
                Err(_) => {
                    ctx.say(format!("Failed to load image: {url}")).await?;
                    return Ok(());
                }
            }
        }
    }

    let message = serenity::CreateMessage::new()
        .content(input.message)
        .add_files(files);

    let message = channel.id.send_message(ctx.http(), message).await?;

    ctx.send(
        poise::CreateReply::default()
            .content("Done")
            .ephemeral(true),
    )
    .await?;

    tracing::info!("Sent message by {}: {}", ctx.author().name, message.id);

    Ok(())
}
