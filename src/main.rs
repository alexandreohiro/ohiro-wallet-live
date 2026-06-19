mod app;
mod auth;
mod errors;
mod models;
mod routes;
mod store;
mod utils;
mod views;

use app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    App::start().await
}
