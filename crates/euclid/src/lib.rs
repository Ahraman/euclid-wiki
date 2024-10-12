use axum::{routing::get, Router};

use self::routing::root;

pub mod error;
pub mod routing;

pub fn make_router() -> Router {
    Router::new().route("/", get(root::view))
}
