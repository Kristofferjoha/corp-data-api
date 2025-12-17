CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    birth_date DATE NOT NULL CHECK (birth_date < CURRENT_DATE),
    office_id INT NOT NULL REFERENCES offices(id)
);
