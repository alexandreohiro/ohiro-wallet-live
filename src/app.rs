use std::{
    env,
    sync::{Arc, Mutex},
};

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

use crate::{routes, store::MemoryStore};

#[derive(Clone)]
pub struct AppState {
    pub memory: Arc<Mutex<MemoryStore>>,
    pub config: AppConfig,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub api_secret_key: String,
    pub seed_demo_data: bool,
}

impl AppConfig {
    pub fn from_env() -> color_eyre::Result<Self> {
        dotenvy::dotenv().ok();

        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(3000);
        let jwt_secret =
            env::var("JWT_SECRET").unwrap_or_else(|_| "local-dev-jwt-secret-change-me".to_string());
        let api_secret_key =
            env::var("API_SECRET_KEY").unwrap_or_else(|_| "dev-secret".to_string());
        let seed_demo_data = env::var("SEED_DEMO_DATA")
            .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(true);

        Ok(Self {
            host,
            port,
            jwt_secret,
            api_secret_key,
            seed_demo_data,
        })
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        init_tracing();
        let config = AppConfig::from_env()?;
        let mut memory = MemoryStore::default();

        if config.seed_demo_data {
            memory.seed_demo();
        }

        let state = AppState {
            memory: Arc::new(Mutex::new(memory)),
            config,
        };
        let app: Router = routes::router(state.clone());
        let address = format!("{}:{}", state.config.host, state.config.port);
        let listener = TcpListener::bind(&address).await?;

        info!(%address, "starting ohiro wallet service");
        axum::serve(listener, app).await?;
        Ok(())
    }
}

fn init_tracing() {
    let layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::NEW)
        .boxed();
    let _ = tracing_subscriber::registry().with(layer).try_init();
}
