use axum::http::HeaderMap;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use time::{Duration, OffsetDateTime};

use crate::{app::AppState, errors::AppError, models::UserRecord, store};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i64,
    username: String,
    name: String,
    exp: i64,
}

pub fn create_session_cookie(user: &UserRecord, secret: &str) -> Result<String, AppError> {
    let claims = Claims {
        sub: user.id,
        username: user.username.clone(),
        name: user.name.clone(),
        exp: (OffsetDateTime::now_utc() + Duration::hours(12)).unix_timestamp(),
    };

    let header = serde_json::json!({"alg":"HS256","typ":"JWT"});
    let header =
        URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).map_err(|_| AppError::Internal)?);
    let payload =
        URL_SAFE_NO_PAD.encode(serde_json::to_vec(&claims).map_err(|_| AppError::Internal)?);
    let message = format!("{}.{}", header, payload);
    let signature = sign(message.as_bytes(), secret.as_bytes())?;
    let signature = URL_SAFE_NO_PAD.encode(signature);

    Ok(format!("{}.{}", message, signature))
}

pub async fn current_user(headers: &HeaderMap, state: &AppState) -> Result<UserRecord, AppError> {
    let token = session_token_from_headers(headers).ok_or(AppError::Unauthorized)?;
    let claims = verify_session_token(&token, &state.config.jwt_secret)?;
    store::find_user_by_id(state, claims.sub)
        .await?
        .ok_or(AppError::Unauthorized)
}

fn verify_session_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(AppError::Unauthorized);
    }

    let message = format!("{}.{}", parts[0], parts[1]);
    let expected = URL_SAFE_NO_PAD.encode(sign(message.as_bytes(), secret.as_bytes())?);

    if expected != parts[2] {
        return Err(AppError::Unauthorized);
    }

    let payload = URL_SAFE_NO_PAD
        .decode(parts[1])
        .map_err(|_| AppError::Unauthorized)?;
    let claims: Claims = serde_json::from_slice(&payload).map_err(|_| AppError::Unauthorized)?;

    if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
        return Err(AppError::Unauthorized);
    }

    Ok(claims)
}

fn sign(message: &[u8], secret: &[u8]) -> Result<Vec<u8>, AppError> {
    let mut mac = HmacSha256::new_from_slice(secret).map_err(|_| AppError::Internal)?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}

fn session_token_from_headers(headers: &HeaderMap) -> Option<String> {
    let raw = headers.get(axum::http::header::COOKIE)?.to_str().ok()?;
    raw.split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix("wallet_session=").map(str::to_string))
}

pub fn session_set_cookie(token: &str) -> String {
    format!(
        "wallet_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=43200",
        token
    )
}

pub fn session_clear_cookie() -> &'static str {
    "wallet_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"
}
