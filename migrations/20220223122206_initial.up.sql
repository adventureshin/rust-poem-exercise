CREATE TABLE user_table (
  id SERIAL PRIMARY KEY ,
  username VARCHAR NOT NULL UNIQUE,
  password_hash VARCHAR NOT NULL,
  is_superuser INTEGER NOT NULL
);
