use std::sync::Arc;

use axum::routing::post;
use axum::{Router, middleware};
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use tokio::sync::RwLock;

use crate::events::event_handler;
use crate::middlewares::api_key::ApiKey;

pub mod commands;
pub mod controllers;
pub mod descriptions;
pub mod embeds;
pub mod events;
pub mod graphql;
pub mod middlewares;
pub mod utils;

#[derive(Default)]
pub struct AppState {
    pub discord: RwLock<Option<serenity::Context>>,
}

pub struct Data {
    pub state: Arc<AppState>,
} // User data, which is stored and accessible in all command invocations

type Error = anyhow::Error;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is required in .env");
    let intents = serenity::GatewayIntents::all();

    let state = Arc::new(AppState::default());
    let framework_state = Arc::clone(&state);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // Slash Commands
                commands::say::say(),
                commands::video::video(),
                // Menu Commands
                commands::edit::edit_message(),
            ],
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    state: framework_state,
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    let api_key = ApiKey(std::env::var("API_KEY").expect("API_KEY is required in .env"));

    let app = Router::new()
        .route(
            "/notification",
            post(controllers::video_notification::video_notification).route_layer(
                middleware::from_fn_with_state(api_key, middlewares::api_key::validate),
            ),
        )
        .with_state(state);

    let bind = format!(
        "0.0.0.0:{}",
        std::env::var("API_PORT").unwrap_or("3000".to_string())
    );

    tracing::debug!("HTTP attempting to bind {}", bind);

    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();

    tracing::info!("HTTP server listening on {}", listener.local_addr().unwrap());

    tokio::select! {
        result = client.start() => {
            tracing::error!("Discord client stopped: {:?}", result);
            result?;
        }
        result = axum::serve(listener, app) => {
            tracing::error!("HTTP server stopped: {:?}", result);
            result?;
        }
    }

    Ok(())
}
