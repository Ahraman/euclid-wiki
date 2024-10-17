use axum::response::{Html, IntoResponse, Response};
use serde_json::json;

use crate::error::Error;

use super::App;

mod model {
    pub(super) struct Page {
        pub(super) revision: i32,
    }

    impl Page {
        pub(super) async fn query(
            title: &str,
            conn: impl sqlx::PgExecutor<'_>,
        ) -> Result<Option<Self>, super::Error> {
            Ok(sqlx::query_as!(
                Page,
                r#"SELECT page_rev AS revision
                    FROM pages
                    WHERE page_title = $1"#,
                &title
            )
            .fetch_optional(conn)
            .await?)
        }
    }

    pub(super) struct Revision {
        pub(super) content: i32,
    }

    impl Revision {
        pub(super) async fn query(
            id: i32,
            conn: impl sqlx::PgExecutor<'_>,
        ) -> Result<Self, super::Error> {
            Ok(sqlx::query_as!(
                Revision,
                r#"SELECT rev_content AS content
                    FROM revisions
                    WHERE rev_id = $1"#,
                id
            )
            .fetch_one(conn)
            .await?)
        }
    }

    pub(super) struct Content {
        pub(super) body: String,
    }

    impl Content {
        pub(super) async fn query(
            id: i32,
            conn: impl sqlx::PgExecutor<'_>,
        ) -> Result<Self, super::Error> {
            Ok(sqlx::query_as!(
                Content,
                r#"SELECT content_body AS body
                    FROM content
                    WHERE content_id = $1"#,
                id
            )
            .fetch_one(conn)
            .await?)
        }
    }
}

impl App {
    pub async fn view_page(&self, title: &str) -> Result<Response, Error> {
        let conn = &self.conn_pool;
        let Some(page) = model::Page::query(title, conn).await? else {
            return Ok(Html::from(
                self.handlebars
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
            )
            .into_response());
        };

        let revision = model::Revision::query(page.revision, conn).await?;
        let content = model::Content::query(revision.content, conn).await?;
        Ok(Html::from(
            self.handlebars
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
        )
        .into_response())
    }

    pub async fn edit_page(&self, title: &str) -> Result<Response, Error> {
        let conn = &self.conn_pool;

        let content = match model::Page::query(title, conn).await? {
            Some(page) => {
                let revision = model::Revision::query(page.revision, conn).await?;
                let content = model::Content::query(revision.content, conn).await?;

                Some(content.body)
            }
            None => None,
        }
        .unwrap_or_default();

        Ok(Html::from(
            self.handlebars
                .read()
                .expect("RwLock poisoning for handlebars")
                .render(
                    "edit",
                    &json!({
                        "page": {
                            "title": &title,
                            "content": &content,
                        }
                    }),
                )?,
        )
        .into_response())
    }

    pub async fn submit_page(&self, title: &str, content: &str) -> Result<Response, Error> {
        /*let conn = &self.conn_pool;

                let _ = sqlx::query!(
        r#"INSERT INTO content (content_body)
            VALUES ($1)"#,
        content)
        .execute(conn)
        .await?;*/

        todo!()
    }
}
