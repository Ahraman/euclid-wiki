use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};

use crate::{app::App, error::Error};

pub async fn get(State(app): State<Arc<App>>) -> Result<impl IntoResponse, Error> {
    app.reload_handlebars()?;

    Ok(Redirect::to("/"))
}
