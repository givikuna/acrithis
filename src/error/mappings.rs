use std::io::Error as IoError;

use rusqlite::Error as SqliteError;

use super::core::AcrithisError;

use crate::shared::types::structures::graph::GraphError;
use crate::shared::types::structures::tree::TreeError;
use crate::store::hashing::error::HashingError;


impl From<GraphError> for AcrithisError {
    fn from(err : GraphError) -> Self {
        match err {
            | GraphError::MissingDependency(id) => {
                AcrithisError::graph(format!("missing dependency: {}", id))
            },
            | GraphError::SelfLoop(id) => {
                AcrithisError::graph(format!("self-loop detected: {}", id))
            },
            | GraphError::ClosureDepthMismatch { expected, got, } => {
                AcrithisError::graph(format!("closure_depth mismatch: expected {}, got {}",
                                             expected, got))
            },
            | GraphError::Conflict(id) => AcrithisError::conflict(format!("node conflict: {}", id)),
            | GraphError::NotFound(id) => {
                AcrithisError::not_found(format!("node not found: {}", id))
            },
        }
    }
}

impl From<TreeError> for AcrithisError {
    fn from(err : TreeError) -> Self {
        match err {
            | TreeError::NotFound(id) => {
                AcrithisError::not_found(format!("tree node not found: {}", id))
            },
            | TreeError::Conflict(id) => {
                AcrithisError::conflict(format!("tree node conflict: {}", id))
            },
            | TreeError::MultipleParents(id) => {
                AcrithisError::graph(format!("multiple parents for tree node: {}", id))
            },
            | TreeError::Cycle { parent, child, } => {
                AcrithisError::graph(format!("cycle detected: {} -> {}", parent, child))
            },
        }
    }
}

impl From<HashingError> for AcrithisError {
    fn from(err : HashingError) -> Self {
        match err {
            | HashingError::Io(e) => AcrithisError::io("IO error during hashing").with_cause(e),
            | HashingError::InvalidLength(len) => {
                AcrithisError::hash(format!("invalid hash length: expected 64, got {}", len))
            },
            | HashingError::InvalidHex => {
                AcrithisError::hash("hash contains invalid non-hex characters")
            },
        }
    }
}

impl From<IoError> for AcrithisError {
    fn from(err : IoError) -> Self { AcrithisError::io("IO operation failed").with_cause(err) }
}

impl From<SqliteError> for AcrithisError {
    fn from(err : SqliteError) -> Self {
        match err {
            | SqliteError::SqliteFailure(err_code, Some(msg))
                if msg.contains("UNIQUE constraint failed") =>
            {
                AcrithisError::conflict("database unique constraint violation").with_cause(msg)
            },
            | SqliteError::QueryReturnedNoRows => {
                AcrithisError::not_found("database query returned no rows")
            },
            | _ => AcrithisError::database("database operation failed").with_cause(err),
        }
    }
}
