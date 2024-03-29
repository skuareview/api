CREATE TABLE users (
  id UUID PRIMARY KEY,
  email VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  id_role INT,
  CONSTRAINT fk_role
    FOREIGN KEY(id_role)
      REFERENCES roles(id)
)