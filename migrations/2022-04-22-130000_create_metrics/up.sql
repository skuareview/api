CREATE TABLE metrics (
  id SERIAL PRIMARY KEY,
  load_average_1 VARCHAR,
  load_average_2 VARCHAR,
  load_average_3 VARCHAR,
  memory_used VARCHAR,
  memory_total VARCHAR,
  cpu_temp VARCHAR,
  cpu_load VARCHAR,
  id_agent INT,
  CONSTRAINT fk_agent
    FOREIGN KEY(id_agent)
      REFERENCES agents(id)
)