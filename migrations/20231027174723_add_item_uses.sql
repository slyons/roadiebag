-- Add migration script here
CREATE TABLE IF NOT EXISTS item_uses (
    id                  INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    item_id             INTEGER NOT NULL,
    extraction_time     TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    num_rounds          INTEGER NOT NULL,
)