ALTER TABLE IF EXISTS revisions
    ADD CONSTRAINT fk_revisions_pages
        FOREIGN KEY (rev_page) REFERENCES pages (page_id);
