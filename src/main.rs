mod config;
mod error;
mod middlewares;
mod modals;
mod routers;
mod utils;

use crate::{config::get_config, routers::create_app};

use axum::Extension;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Error, Pool, Sqlite, SqlitePool};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = get_config();
    let pool = setup_db(&config.database_url).await.unwrap();
    let app = create_app().layer(Extension(pool));

    const ADDR: &'static str = "127.0.0.1:8989";
    let listener = TcpListener::bind(ADDR).await.unwrap();
    info!("{:<12} - {ADDR}", "LISTENING");
    axum::serve(listener, app).await.unwrap()
}

async fn setup_db(url: &str) -> Result<Pool<Sqlite>, Error> {
    if !Sqlite::database_exists(url).await.unwrap_or(false) {
        warn!("db does not exist. creating one...");
        Sqlite::create_database(url)
            .await
            .and_then(|_| Ok(info!("successfully created a db")))?
    }

    let pool = SqlitePool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
