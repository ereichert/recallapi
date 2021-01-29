CREATE SCHEMA mementos;

CREATE TABLE mementos.mementos
(
    id UUID PRIMARY KEY,
    prompt TEXT NOT NULL,
    details TEXT NOT NULL
);