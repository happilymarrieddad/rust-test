CREATE TABLE IF NOT EXISTS crates (
    id BIGSERIAL PRIMARY KEY
    ,rustacean_id BIGINT NOT NULL REFERENCES rustaceans(id)
    ,code VARCHAR(64) NOT NULL
    ,name VARCHAR(128) NOT NULL
    ,version VARCHAR(64) NOT NULL
    ,description TEXT
    ,created_at TIMESTAMP DEFAULT NOW() NOT NULL
);