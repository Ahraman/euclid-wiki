use axum::{routing::get, Router};
use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};

use self::{
    error::Error,
    routing::{page, root},
};

pub mod error;
pub mod routing;

pub fn make_router() -> Router {
    Router::new().route("/", get(root::view)).nest(
        "/w",
        Router::new()
            .route("/", get(page::empty))
            .route("/*page", get(page::view)),
    )
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
