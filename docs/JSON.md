# JSON

Acrithis accepts nodes as JSON for insertion.

The flag-based `acrithis add` will simply be a convenient wrapper for this.

## Input Format (insert)

```json
{
    "schema": "acrithis-node-v1",
    "name": "modus_ponens",
    "node_type": "axiom",
    "lean_version": "leanprover/lean4:v4.16.0",
    "lean_source": "axiom modus_ponens : ∀ {P Q : Prop}, P → (P → Q) → Q\n",
    "doc_markdown": "# Modus Ponens\n\nIf P holds and P implies Q, then Q holds.\n",
    "dependencies": [{ "to_hash": "…", "role": "uses" }],
    "tags": ["logic", "classical"]
}
```

This is described in <a href="./Schema.md">./Schema.md</a>.

## Output Format (`get`)

`acrithis get <hash> --format json`:

```json
{
    "schema": "acrithis-node-v1",
    "hash": "a1b2…",
    "name": "modus_ponens",
    "node_type": "axiom",
    "lean_version": "leanprover/lean4:v4.16.0",
    "lean_source": "…",
    "doc_markdown": "…",
    "dependencies": [],
    "closure_depth": 0,
    "size_bytes": 1234,
    "has_sorry": false,
    "created_at": "2026-01-14T12:00:00Z",
    "created_by": "seed",
    "validation_status": "passed",
    "trust_level": "axiom",
    "tags": ["logic", "classical"]
}
```

## Export Format (JSONL)

TO-DO

## Determinism

The canonical bytes for hashing are NOT JSON bytes.
Key order and whitespace do not affect the hash.
`meta.json` i written with sorted keys and LF newlines so diffs are stables.
