CREATE TABLE roles (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
)

INSERT INTO roles (id,name) VALUES (1, 'USER');
INSERT INTO roles (id,name) VALUES (2, 'ADMIN');