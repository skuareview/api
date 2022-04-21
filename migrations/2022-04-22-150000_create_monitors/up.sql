CREATE TABLE monitors (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  aws_eventbridge_region VARCHAR NOT NULL,
  aws_eventbridge_name VARCHAR NOT NULL,
  aws_eventbridge_description VARCHAR NOT NULL,
  aws_eventbridge_event_bus_name VARCHAR NOT NULL,
  aws_eventbridge_schedule_expression VARCHAR NOT NULL,
  id_agent INT,
  id_lambda INT,
  id_organization INT,
  CONSTRAINT fk_agent
    FOREIGN KEY(id_agent)
      REFERENCES agents(id),
  CONSTRAINT fk_lambda
    FOREIGN KEY(id_lambda)
      REFERENCES lambdas(id),
  CONSTRAINT fk_organization
    FOREIGN KEY(id_organization)
      REFERENCES organizations(id)
)