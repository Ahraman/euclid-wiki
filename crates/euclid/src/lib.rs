use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};

use self::{
    app::App,
    error::Error,
    routing::{asset, page, reload, root},
};

pub mod app;
pub mod error;
pub mod routing;

pub fn make_router(app: App) -> Router {
    Router::new()
        .route("/", get(root::get))
        .route("/asset", get(asset::get))
        .route("/reload", get(reload::get))
        .nest(
            "/w",
            Router::new()
                .route("/", get(page::root))
                .route("/*page", get(page::get)),
        )
        .with_state(Arc::new(app))
}

pub async fn connect_database(url: &str) -> Result<PgPool, Error> {
    println!("Connecting to database at: {url}");

    if !Postgres::database_exists(url).await? {
        println!("Database not found; creating a new one...");

        Postgres::create_database(url).await?;
    }

    let conn = PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&conn).await?;
    Ok(conn)
}
