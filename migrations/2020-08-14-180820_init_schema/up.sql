CREATE TYPE media_type AS ENUM ('movie', 'series', 'episode', 'clip');
CREATE TYPE library_location AS ENUM ('local', 'ssh');

CREATE TABLE libraries (
    name TEXT PRIMARY KEY,
    media_type media_type NOT NULL,
    path TEXT NOT NULL,
    location library_location NOT NULL
);
