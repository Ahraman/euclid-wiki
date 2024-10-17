use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{app::App, error::Error};

pub async fn get(
    State(app): State<Arc<App>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response, Error> {
    let Some(path) = query.get("path") else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };
    let kind = query.get("kind").map(|s| s.as_str());
    app.load_asset(path, kind).await
}
