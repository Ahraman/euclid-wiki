use std::sync::Arc;

use axum::{extract::State, response::Response};

use crate::{app::App, error::Error};

pub async fn get(State(app): State<Arc<App>>) -> Result<Response, Error> {
    app.reload().await
}
