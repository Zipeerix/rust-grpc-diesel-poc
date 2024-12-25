rm -f .env
echo DATABASE_URL=postgres://rust_poc:rust_poc@localhost:6050/rust_poc >> .env
docker run --name rust-poc-postgres --rm -e POSTGRES_USER=rust_poc -e POSTGRES_PASSWORD=rust_poc -p 6050:5432 -it postgres:14.1-alpine
