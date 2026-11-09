# Error Taxonomy

Every failure in Acrithis is classified.
This gives the CLI stable exit codes, gives `validation_log.detail` a machine-parsable prefix, and lets agents distinguish different kinds of errors.

## Categories

| Code    | Family     | Cause                                                      | Retryable |
| ------- | ---------- | ---------------------------------------------------------- | --------- |
| `EUSG`  | Usage      | Bad args, unknown flag, missing required option            | no        |
| `ECFG`  | Config     | Missing/invalid config, unknown schema version             | no        |
| `EIO`   | IO         | Cannot read input file, cannot write store dir, fsync fail | maybe     |
| `EDB`   | Database   | SQLite error, migration failure, version downgrade         | no        |
| `EHASH` | Hash       | Computed hash ≠ stored hash, corrupt payload               | no        |
| `EGRF`  | Graph      | Missing dependency, self-loop, closure_depth mismatch      | no        |
| `EVAL`  | Validation | Lean compile failed, `sorry` present, unexpected axiom     | maybe     |
| `EPRM`  | Permission | Append-only violation, agent trying to write as human      | no        |
| `ENFD`  | Not Found  | `get`/`deps`/`verify` on unknown hash                      | no        |
| `ECNF`  | Conflict   | Node hash already exists (identical content is idempotent) | no        |

## Message Format

Stderr messages are single lines, prefix-first:

```
EUSG: --type must be one of axiom|theorem|strategy
EGRF: dependency 3f9c… not found in store
EHASH: node a1b2… recomputed hash does not match
EDB: schema_version 3 > supported 1 (downgrade unsupported)
EVAL: sorry at line 14
EVAL: axiom myAxiom declared but node is not type=axiom
EGRF: closure_depth mismatch: expected 3, got 2
...
```

## Exit Codes

Acrithis uses a small set of exist codes for families of error categories.

| Exit | Meaning               | Families        |
| ---- | --------------------- | --------------- |
| 0    | success               | —               |
| 1    | generic runtime error | `EIO`, `EDB`    |
| 2    | usage error           | `EUSG`, `ECFG`  |
| 3    | validation failed     | `EVAL`          |
| 4    | store / db integrity  | `EHASH`, `EGRF` |
| 5    | not found             | `ENFD`          |
| 6    | conflict              | `ECNF`          |
| 7    | permission denied     | `EPRM`          |

## Logging

- `--verbose` adds the underlying cause (SQLite message, Lean stderr trail).
- `--quiet` suppresses everything except the exit code and fatal errors.

Validation logs keep the last 4KiB of lean stderr, not the whole thing, to bound `validation_log` row size.
