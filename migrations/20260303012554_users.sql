-- Up
CREATE TABLE users (
  id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
  name TEXT NOT NULL
);
