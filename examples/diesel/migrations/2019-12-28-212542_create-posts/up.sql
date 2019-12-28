-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);

INSERT INTO posts (title, body, published)
VALUES ('First post', 'Interesting body', 't'),
       ('Second post', 'Very interesting post', 't'),
       ('Draft post', 'This is post will not be shown', 'f');