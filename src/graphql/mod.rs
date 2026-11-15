use std::sync::LazyLock;

use anyhow::{Result, bail};
use cynic::serde::Serialize;
use cynic::serde::de::DeserializeOwned;
use cynic::{GraphQlResponse, QueryBuilder, QueryVariables};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};

#[cynic::schema("animethemes")]
pub(crate) mod schema {}

pub mod video_notification;

pub async fn graphql_query<Q, V>(variables: V) -> Result<GraphQlResponse<Q>>
where
    Q: QueryBuilder<V> + DeserializeOwned,
    V: QueryVariables + Serialize,
{
    static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
        let mut headers = HeaderMap::new();

        headers.insert(USER_AGENT, HeaderValue::from_static("AnimeThemesBot/1.0"));

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client")
    });

    let url = std::env::var("GRAPHQL_URL").expect("GRAPHQL_URL is required in .env");

    let operation = Q::build(variables);

    let response: GraphQlResponse<Q> = CLIENT
        .post(url)
        .json(&operation)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if let Some(errors) = response.errors.as_ref().filter(|e| !e.is_empty()) {
        bail!("GraphQL errors: {errors:?}");
    }

    Ok(response)
}
