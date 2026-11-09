# SQLite

Acrithis uses SQLite as the backend for the actual database.
The store directory is authoritative for bytes; SQLite is authoritative for graph structure and metadata.
They must be kept consistent.

## Location

`$ACRITHIS_HOME/acrithis.db`, default `~/.acrithis/acrithis.db`.

Tests set `ACRITHIS_HOME` to a temp directory.

## Pragmas (applied on every connection)

```sql
PRAGMA journal_mode = WAL;      -- concurrent readers with one writer
PRAGMA foreign_keys = ON;       -- enforce REFERENCES
PRAGMA synchronous = NORMAL;    -- safe with WAL, faster than FULL
PRAGMA busy_timeout = 5000;     -- 5 s, then error
```

WAL is required:the store is read-heavy and the CLI may run concurrently with a validation daemon.

## Schema Versioning

`schema_meta` holds a single `schema_version` integer.
On open:

- New DB -> apply migrations `0 -> current` in a transaction
- `db_version < current` -> apply pending migrations in a transaction
- `deb_version > current` -> refuse to open (downgrade unsupported)

Migrations live at `src/store/sql/migrations/NNNN_short_name.sql` and are applied in lexicographic order.

## Append-Only Enforcement

The application layer never issues `UPDATE` or `DELETE` against `nodes`, `dependencies`, `tags`, or `validation_log`.

## Write Ordering (Crash Safety)

The directory is written first, then DB row:

- `store/<aa>/<hash>.tmp/` written and fsynced
- `rename(2)` into `store/<aa>/<hash>/`
- `BEGIN IMMEDIATE` ... insert ... `COMMIT`

A crash leaves at worst an orphan directory with no DB row.
`acrithis gc` removes those.
The reverse order (`DB First`) would leave the DB rows pointing at missing bytes, which is worse.

## Transactions

Every write is one transaction.
`BEGIN IMMEDIATE` ensures we take the write lock before reading for validation.
Avoiding a TOCTOU between the DAG check and the insert.

## Connection Lifecycle

CLI: one connection per invocation. Validation daemon (v0.0.2+): small pool.
No global state, no `OnceCell<Connection>`.

## Query Patterns

TO-DO

## Indexes

Defined in <a href="./Schema.md">`./Schema.md`</a>.

## Backup and Export

TO-DO
