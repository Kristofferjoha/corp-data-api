CREATE TABLE offices (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    max_occupancy INT NOT NULL CHECK (max_occupancy > 0)
);