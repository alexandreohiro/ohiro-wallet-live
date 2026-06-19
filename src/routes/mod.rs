use axum::{
    routing::{get, post},
    Router,
};

use crate::app::AppState;

pub mod api;
pub mod web;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(web::index))
        .route("/login", get(web::login_page).post(web::login))
        .route("/register", get(web::register_page).post(web::register))
        .route("/logout", post(web::logout))
        .route("/assets", get(web::assets_page).post(web::create_asset))
        .route("/purchases", post(web::create_purchase))
        .route("/api/assets", get(api::list_assets).post(api::create_asset))
        .with_state(state)
}
