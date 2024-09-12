-- Add migration script here
CREATE TABLE clients (
    client_id  SERIAL PRIMARY KEY,
    domain varchar(511) NOT NULL UNIQUE,
    email varchar(255) NOT NULL UNIQUE,
    detail text NOT NULL,
    created_on timestamp with time zone NOT NULL,
    updated_on timestamp with time zone NOT NULL
);
