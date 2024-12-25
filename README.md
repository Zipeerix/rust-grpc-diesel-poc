1. Download & install Rust and dependencies

```
sudo apt-get install libpq-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Start database

This will start a postgres container with empty database and create '.env' file so the application can connect to it.

```
chmod +x start_db_in_docker.sh
./start_db_in_docker.sh
```

3. Start application

Database migrations will run automatically on startup. To start the applicaton use:

```
cargo run -- <command line arguments here>
```

By default grpc service will start at `http://127.0.0.1:4000` and prometheus metrics at `http://127.0.0.1:4001`.

Startup example:

```
cargo run -- --config conf/default.toml
```