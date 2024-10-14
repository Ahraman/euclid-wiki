use euclid::{app::App, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenvy::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")?;
    let conn_pool = euclid::connect_database(&database_url).await?;
    println!("Database connection established.");

    let app = App::new(conn_pool)?;
    let router = euclid::make_router(app);

    let server_url = std::env::var("SERVER_URL")?;
    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    println!("Starting server. Listening at: {server_url}");

    axum::serve(listener, router).await?;
    Ok(())
}
