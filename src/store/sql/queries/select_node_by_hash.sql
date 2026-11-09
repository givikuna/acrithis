SELECT
    hash,
    name,
    node_type,
    lean_source_sha,
    doc_sha,
    lean_version,
    closure_depth,
    size_bytes,
    has_sorry,
    trust_level,
    validation_status,
    created_at,
    created_by
FROM
    nodes
WHERE
    hash = ?1;