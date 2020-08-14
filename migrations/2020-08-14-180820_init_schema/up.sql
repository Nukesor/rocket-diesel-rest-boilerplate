CREATE TYPE media_type AS ENUM ('movie', 'series', 'episode', 'clip');

CREATE TABLE libraries (
    name TEXT PRIMARY KEY,
    media_type media_type NOT NULL,
    path TEXT NOT NULL
);
