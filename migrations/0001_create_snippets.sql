CREATE TABLE IF NOT EXISTS snippets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    title TEXT NOT NULL,

    description TEXT NOT NULL DEFAULT '',

    language TEXT NOT NULL,

    code TEXT NOT NULL,

    tags TEXT NOT NULL DEFAULT '',

    is_favorite INTEGER NOT NULL DEFAULT 0,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_snippets_language
ON snippets(language);

CREATE INDEX IF NOT EXISTS idx_snippets_favorite
ON snippets(is_favorite);

CREATE INDEX IF NOT EXISTS idx_snippets_created
ON snippets(created_at);
