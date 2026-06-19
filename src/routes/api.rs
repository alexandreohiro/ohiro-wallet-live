use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::json;

use crate::{app::AppState, errors::AppError, models::ApiCreateAsset, store};

pub async fn list_assets(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    require_api_key(&state, &headers)?;
    let user = store::find_user_by_username(&state, "alexandre")
        .await?
        .ok_or(AppError::Unauthorized)?;
    let assets = store::portfolio_for_user(&state, user.id).await?;
    Ok(Json(json!({ "assets": assets })))
}

pub async fn create_asset(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ApiCreateAsset>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    require_api_key(&state, &headers)?;
    let user = store::find_user_by_username(&state, "alexandre")
        .await?
        .ok_or(AppError::Unauthorized)?;
    let asset =
        store::create_asset(&state, user.id, &payload.name, payload.unit_value_cents).await?;
    Ok((StatusCode::CREATED, Json(json!({ "asset": asset }))))
}

fn require_api_key(state: &AppState, headers: &HeaderMap) -> Result<(), AppError> {
    let key = headers
        .get("x-api-key")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if key == state.config.api_secret_key {
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}
