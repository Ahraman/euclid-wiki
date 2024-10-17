use std::path::Path;

use axum::{body::Body, response::Response};
use tokio::{fs::File, io::BufReader};
use tokio_util::io::ReaderStream;

use crate::error::Error;

use super::App;

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

impl App {
    pub async fn load_asset(&self, path: &str, kind: Option<&str>) -> Result<Response, Error> {
        let kind = match kind {
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
}
