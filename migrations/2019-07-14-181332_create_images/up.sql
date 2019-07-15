-- Your SQL goes here
CREATE TABLE images (
  image_id serial PRIMARY KEY,
  format integer NOT NULL,
  data bytea NOT NULL
);
