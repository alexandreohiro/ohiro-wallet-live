use askama::Template;
use axum::{
    extract::{Form, State},
    http::{header::SET_COOKIE, HeaderMap, HeaderValue},
    response::{Html, IntoResponse, Redirect},
};

use crate::{
    app::AppState,
    auth,
    errors::AppError,
    models::{AssetForm, LoginForm, PurchaseForm, RegisterForm},
    store,
    utils::{parse_money_to_cents, parse_quantity_to_milli},
    views::{AssetsTemplate, LoginTemplate, RegisterTemplate},
};

pub async fn index() -> Redirect {
    Redirect::to("/assets")
}

pub async fn login_page() -> Html<String> {
    Html(render(LoginTemplate {
        error: String::new(),
        has_error: false,
    }))
}

pub async fn register_page() -> Html<String> {
    Html(render(RegisterTemplate {
        error: String::new(),
        has_error: false,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    match store::verify_user_password(&state, &form.username, &form.password).await {
        Ok(user) => match auth::create_session_cookie(&user, &state.config.jwt_secret) {
            Ok(token) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    SET_COOKIE,
                    HeaderValue::from_str(&auth::session_set_cookie(&token)).unwrap(),
                );
                (headers, Redirect::to("/assets")).into_response()
            }
            Err(error) => error.into_response(),
        },
        Err(_) => Html(render(LoginTemplate {
            error: "usuario ou senha invalidos".to_string(),
            has_error: true,
        }))
        .into_response(),
    }
}

pub async fn register(
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> impl IntoResponse {
    match store::create_user(&state, &form.name, &form.username, &form.password).await {
        Ok(user) => match auth::create_session_cookie(&user, &state.config.jwt_secret) {
            Ok(token) => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    SET_COOKIE,
                    HeaderValue::from_str(&auth::session_set_cookie(&token)).unwrap(),
                );
                (headers, Redirect::to("/assets")).into_response()
            }
            Err(error) => error.into_response(),
        },
        Err(error) => Html(render(RegisterTemplate {
            error: error.to_string(),
            has_error: true,
        }))
        .into_response(),
    }
}

pub async fn logout() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_static(auth::session_clear_cookie()),
    );
    (headers, Redirect::to("/login"))
}

pub async fn assets_page(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    match assets_response(&state, &headers, None).await {
        Ok(html) => Html(html).into_response(),
        Err(AppError::Unauthorized) => Redirect::to("/login").into_response(),
        Err(error) => error.into_response(),
    }
}

pub async fn create_asset(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<AssetForm>,
) -> impl IntoResponse {
    let user = match auth::current_user(&headers, &state).await {
        Ok(user) => user,
        Err(_) => return Redirect::to("/login").into_response(),
    };

    let unit_value = match parse_money_to_cents(&form.unit_value) {
        Some(value) if value > 0 => value,
        _ => {
            return render_assets_or_redirect(
                &state,
                &headers,
                Some("valor unitario invalido".to_string()),
            )
            .await
        }
    };

    match store::create_asset(&state, user.id, &form.name, unit_value).await {
        Ok(_) => Redirect::to("/assets").into_response(),
        Err(error) => render_assets_or_redirect(&state, &headers, Some(error.to_string())).await,
    }
}

pub async fn create_purchase(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<PurchaseForm>,
) -> impl IntoResponse {
    let user = match auth::current_user(&headers, &state).await {
        Ok(user) => user,
        Err(_) => return Redirect::to("/login").into_response(),
    };

    let bought_for = match parse_money_to_cents(&form.unit_value) {
        Some(value) if value > 0 => value,
        _ => {
            return render_assets_or_redirect(
                &state,
                &headers,
                Some("valor de compra invalido".to_string()),
            )
            .await
        }
    };

    let quantity = match parse_quantity_to_milli(&form.quantity) {
        Some(value) if value > 0 => value,
        _ => {
            return render_assets_or_redirect(
                &state,
                &headers,
                Some("quantidade invalida".to_string()),
            )
            .await
        }
    };

    match store::create_purchase(&state, user.id, form.asset_id, quantity, bought_for).await {
        Ok(_) => Redirect::to("/assets").into_response(),
        Err(error) => render_assets_or_redirect(&state, &headers, Some(error.to_string())).await,
    }
}

async fn render_assets_or_redirect(
    state: &AppState,
    headers: &HeaderMap,
    error: Option<String>,
) -> axum::response::Response {
    match assets_response(state, headers, error).await {
        Ok(html) => Html(html).into_response(),
        Err(AppError::Unauthorized) => Redirect::to("/login").into_response(),
        Err(error) => error.into_response(),
    }
}

async fn assets_response(
    state: &AppState,
    headers: &HeaderMap,
    error: Option<String>,
) -> Result<String, AppError> {
    let user = auth::current_user(headers, state).await?;
    let assets = store::portfolio_for_user(state, user.id).await?;
    let error = error.unwrap_or_default();
    let has_error = !error.is_empty();

    Ok(render(AssetsTemplate {
        user_name: user.name,
        assets,
        error,
        has_error,
    }))
}

fn render<T: Template>(template: T) -> String {
    template
        .render()
        .unwrap_or_else(|_| "erro ao renderizar template".to_string())
}
