use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

#[derive(Clone)]
pub struct ApiKey(pub String);

pub async fn validate(State(api_key): State<ApiKey>, request: Request, next: Next) -> Response {
    let valid = request
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == api_key.0);

    if !valid {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(request).await
}
