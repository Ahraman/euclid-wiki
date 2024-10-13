use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
};

pub async fn empty() -> impl IntoResponse {
    Redirect::permanent("/w/main")
}

pub async fn view(Path(page): Path<String>) -> impl IntoResponse {
    format!("Looking for page '{page}'")
}
