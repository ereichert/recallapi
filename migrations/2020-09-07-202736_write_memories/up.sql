CREATE TABLE mementos (
    mem_id SERIAL PRIMARY KEY, 
    prompt TEXT NOT NULL, 
    details TEXT NOT NULL
);