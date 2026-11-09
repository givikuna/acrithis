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

This is described in
