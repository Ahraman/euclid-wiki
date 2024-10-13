use euclid::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenvy::dotenv()?;

    let server_url = std::env::var("SERVER_URL")?;

    let router = euclid::make_router();
    let listener = tokio::net::TcpListener::bind(server_url).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
