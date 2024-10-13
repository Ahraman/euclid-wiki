use euclid::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenvy::dotenv()?;

    let database_url = std::env::var("DATABASE_URL")?;
    let _database_conn = euclid::connect_database(&database_url).await?;

    println!("Database connection established.");

    let server_url = std::env::var("SERVER_URL")?;
    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    println!("Starting server. Listening at: {server_url}");

    let router = euclid::make_router();
    axum::serve(listener, router).await?;

    Ok(())
}
