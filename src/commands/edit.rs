use poise::{Modal as _, serenity_prelude as serenity};

use crate::{Data, Error};

type Context<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(Debug, poise::Modal)]
#[name = "EDIT MESSAGE"]
struct EditMessageModal {
    #[name = "MESSAGE"]
    #[paragraph]
    #[max_length = 2000]
    message: String,

    #[name = "IMAGE LINK"]
    #[paragraph]
    image_links: Option<String>,
}

/// Edits a message sent by the bot.
#[poise::command(
    guild_only,
    ephemeral,
    context_menu_command = "Edit Message",
    default_member_permissions = "ADMINISTRATOR",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn edit_message(ctx: Context<'_>, mut message: serenity::Message) -> Result<(), Error> {
    if message.author.id != ctx.framework().bot_id() {
        ctx.say("This message is not from the bot.").await?;
        return Ok(());
    }

    tracing::info!(
        "[Request from {}] [edit {}] old: {}",
        ctx.author().id,
        message.id,
        message.content
    );

    let links = message
        .attachments
        .iter()
        .map(|a| a.url.as_str())
        .collect::<Vec<_>>()
        .join(",");

    let defaults = EditMessageModal {
        message: message.content.clone(),
        image_links: if links.is_empty() { None } else { Some(links) },
    };

    let Some(input) = EditMessageModal::execute_with_defaults(ctx, defaults).await? else {
        return Ok(());
    };

    let mut attachments = serenity::EditAttachments::new();

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
            if let Some(existing) = message
                .attachments
                .iter()
                .find(|a| a.url == url || a.proxy_url == url)
            {
                attachments = attachments.keep(existing.id);
                continue;
            }

            match serenity::CreateAttachment::url(ctx.http(), url).await {
                Ok(attachment) => {
                    attachments = attachments.add(attachment);
                }
                Err(e) => {
                    ctx.say(format!("Failed to attach image: {}", e)).await?;
                }
            }
        }
    }

    message
        .edit(
            ctx,
            serenity::EditMessage::new()
                .content(input.message)
                .attachments(attachments),
        )
        .await?;

    tracing::info!(
        "[Request from {}] [edit {}] new: {}",
        ctx.author().id,
        message.id,
        message.content
    );

    ctx.say("Done! Message updated.").await?;

    Ok(())
}
