use axum::{routing::get, Router};

use self::routing::{page, root};

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
