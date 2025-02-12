# Notices

## Update Rust and Cargo

```bash
rustup update
rustc --version
cargo --version
```

## Change Rust Mirror

```bash
[source.crates-io]
replace-with = "tuna"

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/crates.io-index"
```

## migrate

in root folder

```bash
-- create table
sea-orm-cli migrate generate create_user_table
```

## create crate

in root folder

```bash
cargo new crates/CRATE_NAME
```

## Refs

- [Rust](https://rust-classes.com/preface)
- [SeaORM](https://pyk.sh/creating-postgresql-tables-with-rusts-seaorm?source=more_series_bottom_blogs)
- [Auth](https://blog.logrocket.com/using-rust-axum-build-jwt-authentication-api/)
- [Auth](https://codevoweb.com/jwt-authentication-in-rust-using-axum-framework/)
- [Reqwest](https://juejin.cn/post/7409999990283632680)