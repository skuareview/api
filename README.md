# Tips

## Migrations
To run/revert migrations on the test database, we should use the option `--database-url`.
`diesel migration run --database-url=$DATABASE_URL_TEST`