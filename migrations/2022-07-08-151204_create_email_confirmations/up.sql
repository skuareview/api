CREATE TABLE email_confirmations (
  id SERIAL PRIMARY KEY,
  id_user UUID NOT NULL,
  code INT NOT NULL,
  expiration_date TIMESTAMP NOT NULL,
    CONSTRAINT fk_user
    FOREIGN KEY(id_user)
      REFERENCES users(id)
)