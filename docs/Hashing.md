# Hashing

Acrithis iss content-addressed: a node's identity is the BLAKE3 hash.
Two nodes with identical semantic content must hash identically regardless of when or by whom they were added.

## Algorithm

Hashing is to be done by BLAKE3: <a href="https://github.com/BLAKE3-team/BLAKE3">https://github.com/BLAKE3-team/BLAKE3</a>.
This is done upon its canonical bytes.

This is to be used for both file integrity and for node identity, so there is only one hash dependency in the build.

## Hashing Process

### Hashed Data

The node hash commits tot he semantic content of the node, not to its storage metadata.

Included:

- Schema tag `acrithis-node-v1`
- `name`
- `node_type`
- `lean_version`
- `lean_sha` - BLAKE3 of `source.lean`, hex
- `doc_sha` - BLAKE3 of `doc.tex` hex
- sorted dependency list of `(to_hash, role)`

Excluded:

- `hash` itself
- `created_at`, `created_by`
- `validation_status`, `validation_log`
- `trust_level`, `tags`
- `closure_depth`, `size_bytes`, `has_sorry`

## Canonical Bytes

Deterministically:

```typescript
"acrithis-node-v1\0"
"name\0" <name> "\0"
"type\0" <node_type> "\0"
"lean_version\0" <lean_version> "\0"
"lean_sha\0" <64 hex chars> "\0"
"doc_sha\0" <64 hex chars> "\0"
"deps\0" <n decimal> "\0"
for each dependency, sorted ascending by (to_hash, role):
"dep\0" <to_hash> "\0" <role> "\0"
```

All strings are UTF-8, no normalization.
Fields are NUL-terminated and keyed, so the encoding is unambiguous.

## Hash Format

64 lowercase hex characters:
`a1b2c3d4e5f6...`

## Verifying an Existing Node

- Read `source.lean`, compute `lean_sha=blake3(bytes)`
- Read `doc.tex`, compute `doc_sha=blake3(bytes)`
- Load dependencies from DB (or `meta.json`)
- Rebuild canonical bytes and hash
- Compare against `nodes.hash`

This'll be reported as an error in rebuilds of Acrithis database.

## Dry-run Hashing

`acrithis hash <file.lean> --name ... --type .. --dep .. [--doc ...]` compute the would be hash without inserting.
Agents can use this to check whether a proof already exists before generating it.

## Merkle Property

Because a node hash includes the hashes of its dependencies, the hash of a theorem transitively commits to its entire proof tree.
This gives:

- Tamper-evidence (any ancestor change changes the root hash)
- Deduplication (identical subproofs collapse)
- Content-addressed lookup (`store/<aa>/<full-hash>/`)
- Partial verification (verifying a node only requires its transitive deps, which are all local)
