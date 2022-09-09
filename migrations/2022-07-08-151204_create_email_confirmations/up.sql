CREATE TABLE email_confirmations (
  id SERIAL PRIMARY KEY,
  code INT NOT NULL,
  expiration_date DATE NOT NULL
)