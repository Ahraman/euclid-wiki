use axum::response::{IntoResponse, Redirect};

pub async fn get() -> impl IntoResponse {
    Redirect::permanent("/w/main")
}
