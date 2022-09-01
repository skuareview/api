# Wait for PostgreSQL
for d in db db_test; do
  until nc -z $d 5432
  do
      echo "Waiting for connection to MySQL database..."
      sleep 2
  done
done

${HOME}/.cargo/bin/diesel migration run &&
${HOME}/.cargo/bin/diesel migration run --database-url=${DATABASE_URL_TEST} &&
${HOME}/.cargo/bin/cargo watch -x run
