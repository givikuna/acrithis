# CLI

Acrithis has a CLI to interact with it.

## Global Flags

| Flag             | Env             | Default       | Meaning                           |
| ---------------- | --------------- | ------------- | --------------------------------- |
| `--store <dir>`  | `ACRITHIS_HOME` | `~/.acrithis` | Root of db + `store/` + `cache/`  |
| `--format <fmt>` | —               | `text`        | `text` or `json`                  |
| `--quiet`        | —               | off           | Errors only                       |
| `--verbose`      | —               | off           | Include cause chain               |
| `--help`         | —               | —             | Usage                             |
| `--version`      | —               | —             | Print `.version` + git short hash |

## Commands

### `acrithis init`

Creates `$ACRITHIS_HOME` if missing, applies migrations `0 -> current`,
seeds an empty database. Idempotent. Refuses if the DB's `schema_version`
is higher than the binary supports.

### `acrithis init [--force]`

`--force` only re-runs pragmas; it does not wipe data.

### `acrithis add`

Insert a node. Validates before inserting (see <a href="./Validation.md">`Validation.md`</a>). Rolls back on failure.

```
acrithis add <file.lean>
--name <name>
--type axiom|theorem|strategy
--dep <hash>[:role] ...
[--doc <file.md>]
[--tag <tag>]...
[--from-json <file.json>]
[--insert-on-fail]
[--created-by <id>]
```

| Option             | Required       | Notes                                                        |
| ------------------ | -------------- | ------------------------------------------------------------ |
| `--name`           | yes            | Human-facing identifier, not unique                          |
| `--type`           | yes            | `axiom` \| `theorem` \| `strategy`                           |
| `--dep`            | for non-axioms | `<hash>` or `<hash>:uses` / `<hash>:extends`. Default `uses` |
| `--doc`            | no             | LaTeX associated with the node                               |
| `--tag`            | no             | Repeatable                                                   |
| `--from-json`      | no             | If set, flags are ignored; JSON is authoritative             |
| `--insert-on-fail` | no             | Insert with `validation_status='failed'` instead of rollback |
| `--created-by`     | no             | Defaults to `$USER`                                          |

On success, prints the hash.
With `--format.json`, prints the full node object per <a href="./JSON.md">`JSON.md`</a>.

`--from-json -` reads from stdin.

### `acrithis hash`

Dry-run. Computes the would-be hash without touching the store.
Agents use this to avoid re-generating existing proofs.

```
acrithis hash <file.lean> --name <name> --type <type> --dep <hash>[:role] ... [--doc <file.md>]
```

Output is the 64-char hex hash on stdout, or `{ "hash": "…" }` with `--json`.

### `acrithis get`

```
acrithis get <hash> [--format json]
```

Text mode prints `source.lean` to stdout. `--format json` prints the full
node object. `--doc` prints the associated markdown instead.

### `acrithis list`

```
acrithis list [--type axiom|theorem|strategy]
[--tag <tag>]...
[--trust <level>]
[--status <status>]
[--limit N] [--offset N]
```

Default order: `created_at DESC`. `--format json` emits a JSON array of
summary objects (`hash`, `name`, `node_type`, `closure_depth`,
`validation_status`, `tags`).

### `acrithis deps <hash>`

Direct dependencies. `--tree` walks transitively, indented. `--format json`
emits `{ "hash": "…", "deps": [{ "hash": "…", "role": "uses" }] }`.

### `acrithis rdeps <hash>`

Direct dependents (reverse edge). Same shapes as `deps`.

### `acrithis graph`

```
acrithis graph [--root <hash>]... [--format json]
```

- `json` emits `{ "nodes": [...], "edges": [{ "from": "...", "to": "...", "role": "..." }] }`.

Without `--root`, emits the whole forest. With `--root`, emits the
transitive closure rooted at each given hash.

### `acrithis verify <hash>`

Recompute payload hashes, rebuild canonical bytes, compare against
`nodes.hash`, re-check dependency existence and closure depth, and (unless
`--no-lean`) re-run Lean validation. Never mutates. Exit 4 on mismatch.

```
acrithis verify <hash> [--no-lean]
```

### `acrithis verify-all`

Same as `verify` over the whole store. Reports a table and a final summary.
`--fail-fast` stops on the first error.

### `acrithis gc`

Removes orphan `.tmp` directories, orphan `store/<aa>/<hash>/` dirs with no
DB row, and (with `--unused`) any node whose reverse closure does not reach
a root. `--dry-run` prints what would be removed.

```
acrithis gc [--dry-run] [--unused]
```

`--unused` is conservative: it removes only nodes with **zero** dependents
that are not axioms and not tagged `seed`.
