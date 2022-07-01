CREATE TABLE users (
  id VARCHAR(36) PRIMARY KEY,
  email VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  id_role INT,
  CONSTRAINT fk_role
    FOREIGN KEY(id_role)
      REFERENCES roles(id)
)