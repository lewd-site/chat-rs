CREATE TABLE user_favorite_files (
  id SERIAL PRIMARY KEY,
  file_id INTEGER NOT NULL REFERENCES files(id),
  user_uuid CHAR(36) NOT NULL
);
