CREATE TABLE connection_strings (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    value VARCHAR NOT NULL,
    description VARCHAR,
    source VARCHAR NOT NULL,
    created_at timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp_with_timezone_text NOT NULL DEFAULT CURRENT_TIMESTAMP
)
