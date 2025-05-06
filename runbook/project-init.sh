#!/bin/zsh

# 1. Create root folder and workspace Cargo.toml
mkdir komorebi && cd komorebi
cat >Cargo.toml <<'EOF'
[workspace]
members = [
  "base",
  "lib",
  "utils",
  "client",
  "client/migration",
  "gateway",
  "gateway/migration",
  "price",
  "price/migration",
]
EOF

# 2. Create common crates (base, lib, utils)
cargo new base --lib
cargo new lib --lib
cargo new utils --lib

# 3. Scaffold each service (binary crates) and nested migration crates
for svc in client gateway price; do
    cargo new $svc --bin
    cargo new $svc/migration --lib
done

# 4. Install SeaORM CLI with MySQL support (if not already)
cargo install sea-orm-cli --features mysql --force

# 5. Point your DATABASE_URL
export DATABASE_URL="mysql://backend_user:backend_password@127.0.0.1:13306/komorebi"

# 6. Generate SeaORM entities *inside* each service
sea-orm-cli generate entity -u $DATABASE_URL -o client/src/entities
sea-orm-cli generate entity -u $DATABASE_URL -o gateway/src/entities
sea-orm-cli generate entity -u $DATABASE_URL -o price/src/entities

# 7. Initialize migrations in each service’s migration folder
for svc in client gateway price; do
    (cd $svc/ && sea-orm-cli migrate init)
done

# 8. Generate an initial "create_*_tables" migration for each
sea-orm-cli migrate generate --migration-dir client/migration create_client_tables
sea-orm-cli migrate generate --migration-dir gateway/migration create_gateway_tables
sea-orm-cli migrate generate --migration-dir price/migration create_price_tables

# 9. Run all migrations
for svc in client gateway price; do
    (cd $svc/migration && sea-orm-cli migrate up)
done
