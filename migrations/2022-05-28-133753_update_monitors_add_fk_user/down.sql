ALTER TABLE monitors DROP COLUMN id_user;
-- ALTER TABLE monitors DROP CONSTRAINT fk_user;

-- ALTER TABLE monitors ADD id_user INT;
-- ALTER TABLE monitors ADD CONSTRAINT fk_user
--     FOREIGN KEY(id_user)
--       REFERENCES users(id);