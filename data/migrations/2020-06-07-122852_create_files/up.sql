CREATE TABLE files (
  id SERIAL PRIMARY KEY,
  md5 CHAR(32) NOT NULL,
  size BIGINT NOT NULL,
  name VARCHAR NOT NULL,
  mimetype VARCHAR NOT NULL,
  extension VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL,
  post_id INTEGER NOT NULL REFERENCES posts(id),
  width INTEGER,
  height INTEGER,
  length INTEGER
);
