use rusqlite::{CachedStatement, Transaction, params};

use crate::error::{AcriResult, ResultExt};
use crate::store::hashing::compute::hash_bytes;
use crate::store::models::Node;

const Q_INSERT_NODE : &str = include_str!("../../sql/queries/insert_node.sql");
const Q_INSERT_DEP : &str = include_str!("../../sql/queries/insert_dependency.sql");
const Q_INSERT_TAG : &str = include_str!("../../sql/queries/insert_tag.sql");

pub fn insert_node_graph(tx : &Transaction, node : &Node) -> AcriResult<()> {
    let lean_source_sha : String = hash_bytes(node.lean_source.as_bytes()).to_string();
    let doc_sha : String = hash_bytes(node.doc_tex.as_bytes()).to_string();

    tx.execute(Q_INSERT_NODE, params![node.hash,
                                      node.name,
                                      node.node_type,
                                      lean_source_sha,
                                      doc_sha,
                                      node.lean_version,
                                      node.closure_depth,
                                      node.size_bytes as i64,
                                      node.has_sorry,
                                      node.trust_level,
                                      node.validation_status,
                                      node.created_at,
                                      node.created_by,])
      .map_database(format!("Failed to insert node: {}", node.hash))?;

    let mut stmt_dep : CachedStatement<'_> =
        tx.prepare_cached(Q_INSERT_DEP)
          .map_database("Failed to prepare dependency statement")?;

    for dep in &node.dependencies {
        stmt_dep.execute(params![node.hash, dep.to_hash, dep.role])
                .map_database(format!("Failed to insert dependency {} -> {}",
                                      node.hash, dep.to_hash))?;
    }

    let mut stmt_tag : CachedStatement<'_> = tx.prepare_cached(Q_INSERT_TAG)
                                               .map_database("Failed to prepare tag statement")?;

    for tag in &node.tags {
        stmt_tag.execute(params![node.hash, tag])
                .map_database(format!("Failed to insert tag '{}' for node {}",
                                      tag, node.hash))?;
    }

    Ok(())
}
