CREATE TABLE offices (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    max_occupancy INT NOT NULL CHECK (max_occupancy > 0)
);