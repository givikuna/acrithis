use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct DAG<T> {
    nodes: HashMap<String, T>,
    dependencies_list: HashMap<String, HashSet<String>>,
    dependents_list: HashMap<String, HashSet<String>>,
}

impl<T> DAG<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            dependencies_list: HashMap::new(),
            dependents_list: HashMap::new(),
        }
    }
}
