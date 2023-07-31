-- Your SQL goes here
CREATE TABLE engines (
    id serial PRIMARY KEY,
    engine_name varchar(50) UNIQUE NOT NULL,
    tenant varchar(50) NOT NULL,
    engine_type varchar(50) NOT NULL,
    num_utterances int,
    created_at timestamp NOT NULL DEFAULT NOW()
)