use poise::serenity_prelude as serenity;

use crate::{Data, Error};

pub mod ready;

pub async fn event_handler(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => {
            let ctx = framework.serenity_context.clone();

            *framework.user_data.state.discord.write().await = Some(ctx.clone());

            tokio::spawn(async move {
                if let Err(error) = ready::ready(&ctx).await {
                    println!("[ready] Error: {}", error);
                }
            });
        }
        _ => {}
    };

    Ok(())
}
