CREATE TABLE schema_meta(
    KEY TEXT PRIMARY KEY,
    VALUE TEXT NOT NULL
);

CREATE TABLE nodes(
    hash TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    node_type TEXT NOT NULL CHECK(
        node_type IN(
            'axiom',
            'theorem',
            'strategy'
        )
    ),
    lean_source_sha TEXT NOT NULL,
    doc_sha TEXT NOT NULL,
    lean_version TEXT NOT NULL,
    closure_depth INTEGER NOT NULL CHECK(closure_depth >= 0),
    size_bytes INTEGER NOT NULL,
    has_sorry INTEGER NOT NULL DEFAULT 0CHECK(has_sorry IN(0, 1)),
    trust_level TEXT NOT NULL DEFAULT 'unverified' CHECK(
        trust_level IN(
            'axiom',
            'human_verified',
            'machine_verified',
            'unverified'
        )
    ),
    validation_status TEXT NOT NULL CHECK(
        validation_status IN(
            'pending',
            'passed',
            'failed',
            'errored',
            'skipped'
        )
    ),
    created_at TEXT NOT NULL,
    created_by TEXT NOT NULL
);

CREATE INDEX idx_nodes_name ON nodes(name);

CREATE INDEX idx_nodes_type ON nodes(node_type);

CREATE INDEX idx_nodes_created_at ON nodes(created_at DESC);

CREATE INDEX idx_nodes_depth ON nodes(closure_depth);

CREATE TABLE dependencies(
    from_hash TEXT NOT NULL REFERENCES nodes(hash),
    to_hash TEXT NOT NULL REFERENCES nodes(hash),
    role TEXT NOT NULL DEFAULT 'uses' CHECK(role IN('uses', 'extends')),
    PRIMARY KEY(
        from_hash,
        to_hash,
        role
    ),
    CHECK(from_hash <> to_hash)
);

CREATE INDEX idx_dep_to ON dependencies(to_hash, from_hash);

CREATE INDEX idx_dep_role ON dependencies(role);

CREATE TABLE tags(
    hash TEXT NOT NULL REFERENCES nodes(hash),
    tag TEXT NOT NULL PRIMARY KEY(hash, tag)
);

CREATE INDEX idx_tags_tag ON tags(tag);

CREATE TABLE validation_log(
    hash TEXT NOT NULL REFERENCES nodes(hash),
    attempted_at TEXT NOT NULL,
    status TEXT NOT NULL,
    detail TEXT NOT NULL DEFAULT ''
);

CREATE INDEX idx_vlog_hash ON validation_log(hash, attempted_at DESC);

INSERT INTO
    schema_meta(KEY, VALUE)
VALUES
    ('schema_version', '1');