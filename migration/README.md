# Running Migrator CLI

## Migration

### Generate a new migration file

```sh
cargo run -- generate MIGRATION_NAME
```

### Apply all pending migrations (run in crate folder)

```sh
cargo run
// or
cargo run -- up
```

### Apply first 10 pending migrations

```sh
cargo run -- up -n 10
```

### Rollback last applied migrations

```sh
cargo run -- down
```

### Rollback last 10 applied migrations

```sh
cargo run -- down -n 10
```

### Drop all tables from the database, then reapply all migrations

```sh
cargo run -- fresh
```

### Rollback all applied migrations, then reapply all migrations

```sh
cargo run -- refresh
```

### Rollback all applied migrations

```sh
cargo run -- reset
```

### Check the status of all migrations

```sh
cargo run -- status
```

## Entity

### Create entity lib

```sh
-- run in root directory
cargo new entity --lib
```

### generate entities

```sh
-- run in current directory
sea-orm-cli generate entity -o ../entity/src --lib --with-serde=both --date-time-crate=chrono
```
