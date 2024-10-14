CREATE TABLE IF NOT EXISTS pages (
    page_id SERIAL PRIMARY KEY,
    page_title VARCHAR(255) NOT NULL UNIQUE,

    page_rev INTEGER NOT NULL
        REFERENCES revisions (rev_id),

    page_creator INTEGER
        REFERENCES users (user_id)
            ON DELETE SET NULL,
    page_create_ts TIMESTAMPTZ NOT NULL
);
