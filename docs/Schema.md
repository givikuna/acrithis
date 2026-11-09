# Node Schema

A single immutable unit of the store is a node.

This is the definition for the said Schema.

There are four representations:

| Representation  | Location                       | Authority           |
| --------------- | ------------------------------ | ------------------- |
| Rust `Node`     | in-memory                      | runtime shape       |
| SQLite rows     | `.acrithis/acrithis.db`        | **source of truth** |
| On-disk payload | `.acrithis/store/<aa>/<hash>/` | bytes of content    |
| Canonical bytes | computed transiently           | the hash input      |

--

## Schema Representation

### NodeType

The types of nodes that exist:

```rust
pub enum NodeType {
    Axiom,
    Theorem,
    Strategy, // like induction, pigeonhole, etc...
}
```

(AI cannot append Axioms, for clarification)

### DependencyRole

What kind of a dependency a node is to another node.
There are two options.
`Uses` and `Extends`.
`Uses` - The node uses a declaration from the dependency
`Extends` - The node is a direct wrapper around the dependency

`Uses` is the default.

```rust
pub enum DependencyRole {
    Uses,
    Extends,
}
```

### Dependency

Dependencies of a node.

```rust
pub struct Dependency {
    pub to_hash: String, // 64 lowercase hex chars (per BLAKE3)
    pub role:    DependencyRole,
}
```

### Node

```rust
pub struct Node {
    pub hash: String,

    pub name:      String,
    pub node_type: NodeType,

    pub lean_source:  String,
    pub doc_tex: String,

    pub dependencies:  Vec<Dependency>,
    pub closure_depth: u32.

    pub lean_version: String,

    pub created_at:        DateTime<Utc>,
    pub created_by:        String,
    pub validation_status: ValidationStatus,
    pub trust_level:       u8,
    pub size_bytes:        u64,
    pub tags:              Vec<String>, // tags aren't hashed
}
```

### ValidationStatus

```rust
pub enum ValidationStatus {
    Pending,
    Passed,
    Failed,
    Errored,
    Skipped,
}
```

### TrustLevel

```rust
pub enum TrustLevel {
    Axiom,
    HumanVerified,
    MachineVerified,
    Unverified,
}
```

AI may only append `Theorem`, `Strategy`, and `Unverified` nodes.
The rest are to be done by humans.
Only humans can set nodes as `HumanVerified`.

## In SQLite

```SQL
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE schema_meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

CREATE TABLE nodes (
    hash              TEXT PRIMARY KEY,
    name              TEXT NOT NULL,
    node_type         TEXT NOT NULL
                      CHECK (node_type IN ('axiom','theorem','strategy')),
    lean_source_sha   TEXT NOT NULL,       -- blake3 hex of source.lean
    doc_sha           TEXT NOT NULL,       -- blake3 hex of doc.tex
    lean_version      TEXT NOT NULL,
    closure_depth     INTEGER NOT NULL CHECK (closure_depth >= 0),
    size_bytes        INTEGER NOT NULL,
    has_sorry         INTEGER NOT NULL DEFAULT 0 CHECK (has_sorry IN (0,1)),
    trust_level       TEXT NOT NULL DEFAULT 'unverified'
                      CHECK (trust_level IN
                      ('axiom','human_verified','machine_verified','unverified')),
    validation_status TEXT NOT NULL
                      CHECK (validation_status IN
                      ('pending','passed','failed','errored','skipped')),
    created_at        TEXT NOT NULL,       -- RFC3339, UTC
    created_by        TEXT NOT NULL
);

CREATE INDEX idx_nodes_name       ON nodes(name);
CREATE INDEX idx_nodes_type       ON nodes(node_type);
CREATE INDEX idx_nodes_created_at ON nodes(created_at DESC);
CREATE INDEX idx_nodes_depth      ON nodes(closure_depth);

CREATE TABLE dependencies (
    from_hash TEXT NOT NULL REFERENCES nodes(hash),
    to_hash   TEXT NOT NULL REFERENCES nodes(hash),
    role      TEXT NOT NULL DEFAULT 'uses'
              CHECK (role IN ('uses','extends')),
    PRIMARY KEY (from_hash, to_hash, role),
    CHECK (from_hash <> to_hash)
);

-- for reverse edge queries
CREATE INDEX idx_dep_to   ON dependencies(to_hash, from_hash);
CREATE INDEX idx_dep_role ON dependencies(role);

CREATE TABLE tags (
    hash TEXT NOT NULL REFERENCES nodes(hash),
    tag  TEXT NOT NULL,
    PRIMARY KEY (hash, tag)
);
CREATE INDEX idx_tags_tag ON tags(tag);

CREATE TABLE validation_log (
    hash         TEXT NOT NULL REFERENCES nodes(hash),
    attempted_at TEXT NOT NULL,
    status       TEXT NOT NULL,
    detail       TEXT NOT NULL DEFAULT ''
);
CREATE INDEX idx_vlog_hash ON validation_log(hash, attempted_at DESC);
```

Append-only enforcement is on the application layer.
No `UPDATE` or `DELETE` against `nodes`, `dependencies`, or `tags` is ever issued.

This is specified in <a href="./SQLite.md">`./SQLite.md`</a>

## On-disk Layout

Acrithis stores its data at `~/.acrithis/`.

This is the layout of that directory:

```
.acrithis/
├── acrithis.db                # SQLite (seeded source of truth)
├── store/
│   ├── <aa>/                  # first two hex chars of hash
│   │   └── <hash>/            # full 64-char hash
│   │       ├── source.lean
|   |       ├── doc.tex        # LaTeX proof
│   │       └── meta.json
│   └── ...
└── cache/
    └── olean/
        └── <hash>.olean       # compiled module for this node
```

### Metafile

The metafile `meta.json` is the project of `Node`:

```JSON
{
    "hash": "a1b2…",
    "name": "modus_ponens",
    "node_type": "axiom",
    "lean_version": "leanprover/lean4:v4.16.0",
    "dependencies": [],
    "closure_depth": 0,
    "size_bytes": 1234,
    "has_sorry": false,
    "created_at": "2026-01-14T12:00:00Z",
    "created_by": "seed",
    "validation_status": "passed",
    "tags": ["logic", "classical"],
    "files": { "lean": "source.lean", "doc": "doc.tex" }
}
```

### Atomicity

Each `store/<aa>/<hash>/` directory is written to a sibling `store/<aa>/<hash>.tmp/`.
Fsynced, then `rename(2) d` into place.
A crash leaves at worst a `.tmp` that the next garbage collection'll remove.

The DB insert happens only after the rename succeeds; if the DB insert fails, the directory is orphaned, but essentially harmless.
