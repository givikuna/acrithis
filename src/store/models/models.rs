use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Axiom,
    Definition,
    Lemma,
    Theorem,
    Corollary,
    Conjecture,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Pending,
    Passed,
    Failed,
    Skipped,
    Errored,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub target_id: String,
    pub role: String,
}

pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,

    pub statement_text: String,
    pub lean_source: String,
    pub latex_statement: String,

    pub dependencies: Vec<Dependency>,
    pub closure_depth: i32,

    pub validation_status: ValidationStatus,
    pub has_sorry: bool,

    pub created_at: DateTime<Utc>,
    pub created_by: String,
}
