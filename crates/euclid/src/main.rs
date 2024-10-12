use euclid::{error::Error, make_router};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenvy::dotenv()?;

    let server_url = std::env::var("SERVER_URL")?;

    let router = make_router();
    let listener = tokio::net::TcpListener::bind(server_url).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
