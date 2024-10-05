-- Add migration script here
CREATE TABLE admins (
    id UUID PRIMARY KEY NOT NULL,
    fullname VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL,
    division VARCHAR CHECK (division IN ('Director','Finance','IT','Third Party','Customer Service','Marketing')), 
    role VARCHAR CHECK (role IN ('Supervisor', 'Manager', 'Staff')) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
