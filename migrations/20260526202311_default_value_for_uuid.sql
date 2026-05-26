-- Add migration script here
DROP TABLE IF EXISTS 'user';

CREATE TABLE 'user' (
    user_uuid BLOB PRIMARY KEY NOT NULL
        DEFAULT (UNHEX(CONCAT(
              HEX(RANDOMBLOB(6)), '4',
              SUBSTR(HEX(RANDOMBLOB(2)),0,4),
              FORMAT('%X', 8 + ABS(RANDOM() % 4)),
              SUBSTR(HEX(RANDOMBLOB(8)),0,16))))
            UNIQUE CHECK(LENGTH(user_uuid) = 16),
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT(CURRENT_TIMESTAMP)
);