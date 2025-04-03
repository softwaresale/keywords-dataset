
CREATE TABLE IF NOT EXISTS arxiv_metadata (
    id TEXT PRIMARY KEY,
    submitted TEXT,
    authors TEXT,
    title TEXT,
    comments TEXT,
    journal_ref TEXT,
    doi TEXT,
    categories TEXT
);

CREATE TABLE IF NOT EXISTS arxiv_version (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    arxiv_id TEXT NOT NULL,
    version TEXT,
    created TEXT,
    FOREIGN KEY (arxiv_id) REFERENCES arxiv_metadata(id)
);

CREATE TABLE IF NOT EXISTS paper_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    arxiv_id TEXT NOT NULL,
    abstract TEXT NOT NULL,
    keywords TEXT,
    content TEXT,
    FOREIGN KEY (arxiv_id) REFERENCES arxiv_metadata(id)
);

CREATE TABLE IF NOT EXISTS extraction_result (
    arxiv_id TEXT PRIMARY KEY,
    status_code VARCHAR(32),
    status_msg TEXT
);
