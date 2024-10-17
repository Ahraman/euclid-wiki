use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};

use crate::{app::App, error::Error};

pub async fn root() -> impl IntoResponse {
    Redirect::permanent("/w/main")
}

pub async fn get(
    Path(title): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    State(app): State<Arc<App>>,
) -> Result<Response, Error> {
    match query.get("action").map(|s| s.as_str()) {
        Some("create") | Some("edit") => app.edit_page(&title).await,
        _ => app.view_page(&title).await,
    }
}

pub struct SubmitPage {
    content: String,
}

pub async fn post(
    Path(title): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    Form(form): Form<SubmitPage>,
    State(app): State<Arc<App>>,
) -> Result<Response, Error> {
    match query.get("action").map(|s| s.as_str()) {
        Some("submit") => app.submit_page(&title, &form.content).await,
        _ => app.view_page(&title).await,
    }
}
