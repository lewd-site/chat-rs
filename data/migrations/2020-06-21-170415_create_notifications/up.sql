CREATE TABLE notifications (
  id SERIAL PRIMARY KEY,
  post_id INTEGER NOT NULL REFERENCES posts(id),
  user_uuid CHAR(36) NOT NULL,
  read BOOLEAN NOT NULL
);
