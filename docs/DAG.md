# Directed Acyclic Graph (DAG)

The store is a forest of DAGs.
An edge `A -> B` means "`A` depends on `B`".
This is typed by `DependencyRole`.

Roots are nodes with no dependents; as such, they are axioms.

## DAG vs Tree

Proofs are tree-shaped in principle (dependencies on theorems and axioms), but share these dependencies in practice, so the structure is a DAG.
Acyclicity is non-negotiable: a node cannot participate in its own proof.

## Acyclicity is Structural

Content-addressing makes cycles impossible by construction:

- A node hash is a function of its dependencies' hashes (<a href="./Hashing.md">`Hashing.md`</a>)
- To insert `A` depending on `B`, `B` must already exist (its hash is known)
- To then insert `B` depending on `A`, `A`'s hash must already be known, but `A`'s hash depends on `B` hash, which depends on `A`s hash. Circular.

The only escape is a BLAKE3 collision.

## Application-layer Checks on Insert

Even though cycles are structurally impossible, every insert still validates:

- Every `to_hash` exists in `nodes`
- `from_hash != to_hash` (no self loops)
- `new.closure_depth == 1 + max(dep.closure_depth)` (local consistency check)

If any check fails, the insert is rolled back with a specific error (`ErrorTaxonomy.md`).

## Closure Depth

`closure_depth(n) = 0` if `n` has no dependencies.
`closure_depth(n) = 1 + max(closure_depth(d) for d in dep)` otherwise

Stored in `nodes.closure_depth` at insert time.
Recomputing only by `acrithis verify-all`.
Used for topological ordering in `export`, for the local cycle check above, and for bounding recursion in `graph`.

## Merkle DAG Consequences

These are the same as in <a href="./Hashing.md">`Hashing.md`</a>.

Simply:

- Tamper-evidence (any ancestor change changes the root hash)
- Deduplication (identical subproofs collapse)
- Content-addressed lookup (`store/<aa>/<full-hash>/`)
- Partial verification (verifying a node only requires its transitive deps, which are all local)

## Graph Operations (v0.0.1)

| Command                        | Meaning                              |
| ------------------------------ | ------------------------------------ |
| `acrithis deps <hash>`         | direct dependencies                  |
| `acrithis rdeps <hash>`        | direct dependents (reverse edge)     |
| `acrithis graph --format json` | `{ "nodes": [...], "edges": [...] }` |
