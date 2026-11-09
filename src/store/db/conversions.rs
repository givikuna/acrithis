use rusqlite::Result as SqlResult;
use rusqlite::types::{ToSql, ToSqlOutput};

use crate::store::models::{DependencyRole, NodeType, TrustLevel, ValidationStatus};

impl ToSql for NodeType {
    fn to_sql(&self) -> SqlResult<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(match self {
            | NodeType::Axiom => "axiom",
            | NodeType::Theorem => "theorem",
            | NodeType::Strategy => "strategy",
        }))
    }
}

impl ToSql for ValidationStatus {
    fn to_sql(&self) -> SqlResult<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(match self {
            | ValidationStatus::Pending => "pending",
            | ValidationStatus::Passed => "passed",
            | ValidationStatus::Failed => "failed",
            | ValidationStatus::Errored => "errored",
            | ValidationStatus::Skipped => "skipped",
        }))
    }
}

impl ToSql for TrustLevel {
    fn to_sql(&self) -> SqlResult<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(match self {
            | TrustLevel::Axiom => "axiom",
            | TrustLevel::HumanVerified => "human_verified",
            | TrustLevel::MachineVerified => "machine_verified",
            | TrustLevel::Unverified => "unverified",
        }))
    }
}

impl ToSql for DependencyRole {
    fn to_sql(&self) -> SqlResult<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(match self {
            | DependencyRole::Uses => "uses",
            | DependencyRole::Extends => "extends",
        }))
    }
}
