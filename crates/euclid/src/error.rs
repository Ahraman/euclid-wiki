use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    Http(#[from] axum::http::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    Dotenvy(#[from] dotenvy::Error),

    #[error(transparent)]
    Handlebars(#[from] handlebars::TemplateError),
    #[error(transparent)]
    HandlebarsRender(#[from] handlebars::RenderError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        format!("{self}").into_response()
    }
}
