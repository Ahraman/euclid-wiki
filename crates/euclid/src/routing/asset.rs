use std::{collections::HashMap, path::Path};

use axum::{
    body::Body,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tokio::{fs::File, io::BufReader};
use tokio_util::io::ReaderStream;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssetKind {
    Css,

    Unknown,
}

impl AssetKind {
    fn from_path(path: &Path) -> Self {
        let Some(extension) = path.extension() else {
            return Self::Unknown;
        };

        match extension.to_str() {
            Some("css") => Self::Css,
            _ => Self::Unknown,
        }
    }

    fn content_type(&self) -> &'static str {
        match self {
            AssetKind::Css => "text/css",
            AssetKind::Unknown => "application/octet-stream",
        }
    }
}

pub async fn get(Query(query): Query<HashMap<String, String>>) -> Result<Response, Error> {
    let Some(path) = query.get("path") else {
        return Ok(StatusCode::NOT_FOUND.into_response());
    };

    let kind = match query.get("kind").map(|s| s.as_str()) {
        Some("css") | Some("CSS") => AssetKind::Css,
        _ => AssetKind::from_path(Path::new(path)),
    };

    let reader_stream =
        ReaderStream::new(BufReader::new(File::open(format!("assets/{path}")).await?));
    let stream = reader_stream;

    Ok(Response::builder()
        .header("key", kind.content_type())
        .body(Body::from_stream(stream))?)
}
