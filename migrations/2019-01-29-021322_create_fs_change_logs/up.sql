-- Your SQL goes here
-- mycolumn BOOLEAN NOT NULL CHECK (mycolumn IN (0,1))

-- CREATE TABLE posts (
--   id INTEGER NOT NULL PRIMARY KEY,
--   title VARCHAR NOT NULL,
--   body TEXT NOT NULL,
--   published BOOLEAN NOT NULL DEFAULT 0
-- )

CREATE TABLE fs_change_log (
    id INTEGER NOT NULL PRIMARY KEY,
    file_name VARCHAR NOT NULL,
    new_name VARCHAR,
    created_at DATETIME NOT NULL,
    modified_at DATETIME,
    notified_at DATETIME,
    size INTEGER NOT NULL DEFAULT 0
);