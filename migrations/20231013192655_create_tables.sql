-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
  id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  username      TEXT NOT NULL UNIQUE,
  password      TEXT NOT NULL,
  created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_permissions (
    user_id     INTEGER NOT NULL,
    token       TEXT NOT NULL,
    PRIMARY KEY (user_id, token)
);

CREATE TABLE IF NOT EXISTS bagitems (
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    added_by    INTEGER NOT NULL,
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity    INTEGER,
    size        SMALLINT,
    infinite    BOOLEAN,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);