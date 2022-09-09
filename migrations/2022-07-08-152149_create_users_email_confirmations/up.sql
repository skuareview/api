CREATE TABLE users_email_confirmations (
  id SERIAL PRIMARY KEY,
  id_user UUID,
  id_email_confirmation INT,
  CONSTRAINT fk_user
    FOREIGN KEY(id_user)
      REFERENCES users(id),
  CONSTRAINT fk_email_confirmation
    FOREIGN KEY(id_email_confirmation)
      REFERENCES email_confirmations(id)
)