CREATE TABLE IF NOT EXISTS telebins
(
    id INTEGER PRIMARY KEY NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    ts DATETIME NOT NULL
);
