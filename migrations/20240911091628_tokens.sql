-- Add migration script here
CREATE TABLE tokens (
    token_id  SERIAL PRIMARY KEY,
    scopes jsonb,
    email varchar(255) NOT NULL UNIQUE,
    created_on timestamp with time zone NOT NULL,
    updated_on timestamp with time zone NOT NULL,
    client_id int,
    FOREIGN KEY (client_id) REFERENCES clients(client_id),
    FOREIGN KEY (email) REFERENCES users(email)
);
