CREATE TABLE IF NOT EXISTS revisions (
    rev_id SERIAL PRIMARY KEY,
    rev_parent INTEGER DEFAULT NULL
        REFERENCES revisions (rev_id)
            ON DELETE SET NULL,

    rev_page INTEGER NOT NULL,
    rev_user INTEGER
        REFERENCES users (user_id)
            ON DELETE SET NULL,
    rev_content INTEGER NOT NULL
        REFERENCES content (content_id)
            ON DELETE CASCADE,

    rev_create_ts TIMESTAMPTZ NOT NULL
);
