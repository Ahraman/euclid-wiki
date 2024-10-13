use axum::response::{IntoResponse, Redirect};

pub async fn view() -> impl IntoResponse {
    Redirect::permanent("/w/main")
}
