use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
};
use serde_json::json;

use crate::{app::App, error::Error};

pub async fn empty() -> impl IntoResponse {
    Redirect::permanent("/w/main")
}

pub struct Page {
    revision: i32,
}

pub struct Revision {
    content: i32,
}

pub struct Content {
    body: String,
}

pub async fn get(
    Path(title): Path<String>,
    State(app): State<Arc<App>>,
) -> Result<impl IntoResponse, Error> {
    let Some(page) = sqlx::query_as!(
        Page,
        r#"SELECT page_rev AS revision
            FROM pages
            WHERE page_title = $1"#,
        &title
    )
    .fetch_optional(&app.conn_pool)
    .await?
    else {
        return Ok(Html::from(
            app.handlebars
                .read()
                .expect("RwLock poisoning for handlebars")
                .render(
                    "not-found",
                    &json!({
                        "page": {
                            "title": &title,
                        }
                    }),
                )?,
        ));
    };

    let revision = sqlx::query_as!(
        Revision,
        r#"SELECT rev_content AS content
            FROM revisions
            WHERE rev_id = $1"#,
        page.revision
    )
    .fetch_one(&app.conn_pool)
    .await?;

    let content = sqlx::query_as!(
        Content,
        r#"SELECT content_body AS body
            FROM content
            WHERE content_id = $1"#,
        revision.content
    )
    .fetch_one(&app.conn_pool)
    .await?;

    Ok(Html::from(
        app.handlebars
            .read()
            .expect("RwLock poisoning for handlebars")
            .render(
                "page",
                &json!({
                    "page": {
                        "title": &title,
                        "content": &content.body,
                    }
                }),
            )?,
    ))
}
