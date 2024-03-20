# Diesel with MySQL

## Install Diesel CLI

general: `cargo install diesel_cli`
mysql: `cargo install diesel_cli --no-default-features --features mysql`

## Setup Diesel

1. `echo DATABASE_URL=mysql://YOUR_ACCOUNT:YOUR_PASSWORD@DB_HOST:DB_PORT/DATABASE > .env`
1. `diesel setup`
    - folder: ___migrations___.

## Setup Table Schema

1. `diesel migration generate PURPOSE`, eg: `diesel migration generate create_posts`
    - `up.sql` is migration script
    - `down.sql` is rollback script

1. `diesel migration run`
    - redo: `diesel migration redo`

version control in table: `__diesel_schema_migrations`

```bash
mysql> select * from __diesel_schema_migrations;
+----------------+---------------------+
| version        | run_on              |
+----------------+---------------------+
| 20231102125925 | 2023-11-02 21:22:06 |
+----------------+---------------------+
```

## CRUD

1. add `mod schema;` in `main.rs` or `lib.rs`;

### Insert

### Select

### Update

### Delete

## Reference

### OneToOne

### OneToMany

### ManyToMany
