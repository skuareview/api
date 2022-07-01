CREATE TABLE organizations_users (
  id SERIAL PRIMARY KEY,
  id_organization INT,
  id_user VARCHAR,
  CONSTRAINT fk_organization
    FOREIGN KEY(id_organization)
      REFERENCES organizations(id),
  CONSTRAINT fk_user
    FOREIGN KEY(id_user)
      REFERENCES users(id)
)