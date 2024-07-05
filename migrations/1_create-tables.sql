CREATE TABLE mocks (
    name TEXT PRIMARY KEY NOT NULL,
    response TEXT NOT NULL
);

CREATE TABLE logs (
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    name TEXT NOT NULL,
    method TEXT NOT NULL,
    request TEXT,
    response TEXT NOT NULL
);