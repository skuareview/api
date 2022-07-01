ALTER TABLE monitors ADD id_user VARCHAR(36);
ALTER TABLE monitors ADD 
CONSTRAINT fk_user
    FOREIGN KEY(id_user)
      REFERENCES users(id);