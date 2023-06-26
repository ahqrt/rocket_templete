CREATE TABLE crates (
    id SERIAL PRIMARY KEY,
    rustacean_id INTEGER NOT NULL REFERENCES rustaceans(id),
    code VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL
)