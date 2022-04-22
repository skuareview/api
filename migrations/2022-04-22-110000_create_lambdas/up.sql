CREATE TABLE lambdas (
  id SERIAL PRIMARY KEY,
  aws_lambda_region VARCHAR NOT NULL,
  aws_lambda_arn VARCHAR NOT NULL,
  aws_lambda_id VARCHAR NOT NULL
)