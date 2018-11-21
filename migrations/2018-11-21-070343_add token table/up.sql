-- Your SQL goes here
-- Table Definition ----------------------------------------------

CREATE TABLE tokens (
    id SERIAL NOT NULL PRIMARY KEY,
    user_id integer NOT NULL REFERENCES users(id),
    value text NOT NULL,
    expire_at timestamp without time zone NOT NULL DEFAULT (CURRENT_TIMESTAMP + '1 day'::interval)
);