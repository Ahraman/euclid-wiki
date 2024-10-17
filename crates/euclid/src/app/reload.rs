use axum::response::{IntoResponse, Redirect, Response};

use crate::error::Error;

use super::App;

impl App {
    pub async fn reload(&self) -> Result<Response, Error> {
        self.reload_handlebars()?;

        Ok(Redirect::to("/").into_response())
    }
}
