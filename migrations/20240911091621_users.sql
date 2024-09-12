-- Add migration script here
CREATE TABLE users (
    user_id  SERIAL PRIMARY KEY,
    bbb_id varchar(255),
    email varchar(255) NOT NULL UNIQUE,
    phone varchar(255), -- formatting managed by the server
    data jsonb,
    created_on timestamp with time zone NOT NULL,
    updated_on timestamp with time zone NOT NULL
 );
